extern crate gl;
mod opengl;

pub mod texture;
pub mod systems;
pub mod components;

use std::{
    mem, ptr,
    ffi::CString,
};

use crate::{
    math::{self, Point2, Vector2, Vector3},
    platform::{RenderContext, Context},
};

use self::{
    gl::types::*,
    opengl::*,
};

pub use self::{
    opengl::{resize_viewport, get_proc_address},
    texture::{load_texture, gen_texture},
};

#[derive(Debug, Default)]
pub struct RenderBatch {
    start: usize,
    count: usize,
    texture_id: u32,
}

#[derive(Debug, Default)]
pub struct QuadBuffer {
    array: Vec<f32>,
    batch_data: Vec<RenderBatch>,
}

impl QuadBuffer {
    pub fn new() -> Self {
        QuadBuffer {
            array: Vec::new(),
            batch_data: Vec::new(),
        }
    }
}

pub fn clear_quad_buffer(buffer: &mut QuadBuffer) {
    buffer.array.clear();
    buffer.batch_data.clear();
}


#[derive(Debug, Default)]
struct RenderBuffer {
    quads: QuadBuffer,
    vbo_id: u32,
    shader_id: u32,
}

pub struct Renderer {
    buffers: Vec<RenderBuffer>,
    vao_id: u32,
}

impl Renderer {

    pub fn new() -> Renderer {

        let mut vao = 0;

        let program = opengl::make_temp_shader();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }

        let mut renderer = Renderer {
            buffers: Vec::new(),
            vao_id: vao,
        };

        renderer.new_quad_buffer(program);
        renderer
    }

    pub fn get_quad_buffer_vbo_and_shader(&self) -> (u32, u32) {
        let quad_buffer = self.buffers.first().expect("No first buffer yet");
        (quad_buffer.vbo_id, quad_buffer.shader_id)
    }

    pub fn render(&mut self, window_size: (i32, i32), ctx: &mut RenderContext) {

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        for buffer in &mut self.buffers {
            render_buffer(&buffer.quads, buffer.vbo_id, buffer.shader_id,
                self.vao_id, window_size);
            clear_quad_buffer(&mut buffer.quads);
        }

        ctx.swap_buffers();
    }

    pub fn render_quad_buffer(&mut self, buffer: &mut QuadBuffer, vbo_id: u32, shader_id: u32, window_size: (i32, i32), ctx: &mut RenderContext) {

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        render_buffer(buffer, vbo_id, shader_id, self.vao_id, window_size);
        clear_quad_buffer(buffer);

        ctx.swap_buffers();
    }


    fn new_quad_buffer(&mut self, shader_id : u32) {

        let position_str = CString::new("position").unwrap();
        let color_str = CString::new("color").unwrap();
        let texcoord_str = CString::new("texcoord").unwrap();

        let mut vbo_id = 0;
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl::GenBuffers(1, &mut vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);

            // Specify layout
            let pos_attr = gl::GetAttribLocation(shader_id, position_str.as_ptr());
            let col_attr = gl::GetAttribLocation(shader_id, color_str.as_ptr());
            let tc_attr = gl::GetAttribLocation(shader_id, texcoord_str.as_ptr());

            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                7 * mem::size_of::<GLfloat>() as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(col_attr as GLuint);
            gl::VertexAttribPointer(
                col_attr as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                7 * mem::size_of::<GLfloat>() as i32,
                (2 * mem::size_of::<GLfloat>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(tc_attr as GLuint);
            gl::VertexAttribPointer(
                tc_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                7 * mem::size_of::<GLfloat>() as i32,
                (5 * mem::size_of::<GLfloat>()) as *const GLvoid,
            );
        }
        self.buffers.push( RenderBuffer { quads: QuadBuffer::new(), vbo_id, shader_id});
    }

    fn add_to_buffer(&mut self, array: &mut Vec<f32>, texture_id: u32) {
        let buff : &mut RenderBuffer = self.buffers.first_mut().unwrap();

        let thing_per_vertex = 7;
        let batch_info = RenderBatch {
            start: buff.quads.array.len() / thing_per_vertex,
            count: array.len() / thing_per_vertex,
            texture_id,
        };

        buff.quads.array.append(array);
        buff.quads.batch_data.push(batch_info);
    }

    pub fn quad_as_vec(position: Point2<f32>, size: Vector2<f32>, color: Vector3<f32>) -> Vec<f32> {
        vec![
            position.x, position.y,
            color.x, color.y, color.z,
            0.0, 1.0,
            position.x, position.y + size.y,
            color.x, color.y, color.z,
            0.0, 0.0,
            position.x + size.x, position.y,
            color.x, color.y, color.z,
            1.0, 1.0,
            position.x, position.y + size.y,
            color.x, color.y, color.z,
            0.0, 0.0,
            position.x + size.x, position.y + size.y,
            color.x, color.y, color.z,
            1.0, 0.0,
            position.x + size.x, position.y,
            color.x, color.y, color.z,
            1.0, 1.0,
        ]
    }

    pub fn add_quad(&mut self, position: Point2<f32>, size: Vector2<f32>, color: Vector3<f32>, texture_id: u32) {
        let mut v = Renderer::quad_as_vec(position, size, color);
        self.add_to_buffer(&mut v, texture_id);
    }

    pub fn add_quad_to_buffer(buffer: &mut QuadBuffer, position: Point2<f32>, size: Vector2<f32>, color: Vector3<f32>, texture_id: u32) {
        let mut arr = Renderer::quad_as_vec(position, size, color);

        let thing_per_vertex = 7;
        let batch_info = RenderBatch {
            start: buffer.array.len() / thing_per_vertex,
            count: arr.len() / thing_per_vertex,
            texture_id,
        };

        buffer.array.append(&mut arr);
        buffer.batch_data.push(batch_info);
    }
}

fn render_buffer(buffer: &QuadBuffer, vbo_id: u32, shader_id: u32, vao_id: u32, window_size : (i32, i32)) {

    let out_color_str = CString::new("out_color").unwrap();
    let mvp_str = CString::new("mvp").unwrap();

    unsafe {
        gl::UseProgram(shader_id);
        gl::BindFragDataLocation(shader_id, 0, out_color_str.as_ptr());

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_id);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (buffer.array.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            buffer.array.as_ptr() as *const GLvoid,
            gl::DYNAMIC_DRAW,
        );

        let mvp = math::array4x4(math::ortho(0.0, window_size.0 as f32, window_size.1 as f32, 0.0, 0.0, 1.0));

        let mvp_attr = gl::GetUniformLocation(shader_id, mvp_str.as_ptr());
        gl::UniformMatrix4fv(mvp_attr, 1, gl::FALSE, mvp.as_ptr() as *const f32);

        gl::BindVertexArray(vao_id);
        for batch in &buffer.batch_data {
            gl::ActiveTexture(gl::TEXTURE0);
            bind_texture_by_id(batch.texture_id);
            gl::DrawArrays(gl::TRIANGLES, batch.start as i32, batch.count as i32);
        }
    }
}

