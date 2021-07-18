extern crate gl;
pub mod texture;
pub mod material;

use self::gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::path::Path;

use super::math::{self, Point2, Vector2, Vector3};
use platform::glfw::{Context};
use self::texture::{TextureStorage};

use self::opengl::*;

pub use self::opengl::{resize_viewport};

mod opengl;

#[derive(Debug)]
struct RenderBatch {
    start: usize,
    count: usize,
    texture_id: u32,
}

#[derive(Debug)]
struct RenderBuffer {
    data : Vec<f32>,
    batch_data : Vec<RenderBatch>,
    vbo_id : u32,
    shader_id : u32,
}

pub struct Renderer {
    buffers : Vec<RenderBuffer>,
    textures : TextureStorage,
    vao_id : u32,
}

impl Renderer {

    pub fn new(window : &mut glfw::Window) -> Renderer {
        // Make the window's context current
        window.make_current();

        window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        let mut vao = 0;

        let program = opengl::make_temp_shader();

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        let mut renderer = Renderer { buffers: Vec::new(), textures: TextureStorage::new(), vao_id: vao};
        println!("New renderer for {:?}", window);
        renderer.new_quad_buffer(program);
        renderer
    }

    pub fn render(&mut self, window : &mut glfw::Window) {

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let window_size = window.get_size();

        for buffer in &mut self.buffers {
            Renderer::render_buffer(buffer, self.vao_id, window_size);
            buffer.data.clear();
            buffer.batch_data.clear();
        }

        window.swap_buffers();
    }


    fn render_buffer(buffer: &mut RenderBuffer, vao_id: u32, window_size : (i32, i32)) {

        let out_color_str = CString::new("out_color").unwrap();

        let mvp_str = CString::new("mvp").unwrap();

        unsafe {
            gl::UseProgram(buffer.shader_id);
            gl::BindFragDataLocation(buffer.shader_id, 0, out_color_str.as_ptr());

            gl::BindBuffer(gl::ARRAY_BUFFER, buffer.vbo_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (buffer.data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                buffer.data.as_ptr() as *const GLvoid,
                gl::DYNAMIC_DRAW,
            );

            let mvp = math::array4x4(math::ortho(0.0, window_size.0 as f32, window_size.1 as f32, 0.0, 0.0, 1.0));

            let mvp_attr = gl::GetUniformLocation(buffer.shader_id, mvp_str.as_ptr());
            gl::UniformMatrix4fv(mvp_attr, 1, gl::FALSE, mvp.as_ptr() as *const f32);

            static mut first: bool = true;
            gl::BindVertexArray(vao_id);
            for batch in &buffer.batch_data {
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, batch.texture_id);
                //bind_texture_by_id(batch.texture_id);
                gl::DrawArrays(gl::TRIANGLES, batch.start as i32, batch.count as i32);
                if first {
                    println!("Batch texture_id: {}, start: {}, count: {}", batch.texture_id, batch.start, batch.count);
                }
                gl::BindTexture(gl::TEXTURE_2D, 0);
            }
            first = false;
            gl::BindVertexArray(0);
        }
    }

    fn new_quad_buffer(&mut self, shader_id : u32) {

        let position_str = CString::new("position").unwrap();
        let color_str = CString::new("color").unwrap();
        let texcoord_str = CString::new("texcoord").unwrap();

        let mut vbo = 0;
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

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
        self.buffers.push( RenderBuffer { data: Vec::new(), batch_data: Vec::new(), vbo_id : vbo, shader_id : shader_id });
    }

    fn add_to_buffer(&mut self, data: &mut Vec<f32>, texture_id: u32) {
        let buff : &mut RenderBuffer = self.buffers.first_mut().unwrap();

        let thing_per_vertex = 7;

        let batch_info = RenderBatch {
            start: buff.data.len() / thing_per_vertex,
            count: data.len() / thing_per_vertex,
            texture_id: texture_id,
        };

        buff.data.append(data);
        buff.batch_data.push(batch_info);
    }

    pub fn load_textures<P: AsRef<Path>>(&mut self, path: P) {
        match self.textures.load_textures(path) {
            Err(s) => println!("Failed to load textures: {}", s),
            _ => {},
        }
    }

    pub fn get_texture_id(&self, texture_name: &str) -> u32 {
        if let Some(texture_id) = self.textures.get_texture_id(texture_name) {
            return texture_id;
        }
        0
    }

    pub fn add_quad(&mut self, position: Point2<f32>, size: Vector2<f32>, color: Vector3<f32>, texture_id: u32) {
        let mut v = vec![
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
        ];
        self.add_to_buffer(&mut v, texture_id);
    }

    pub fn get_white_id(&self) -> u32 {
        self.textures.get_white_id()
    }
}

