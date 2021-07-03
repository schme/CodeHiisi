extern crate gl;

use self::gl::types::*;
use std::ffi::CString;
use std::mem;
use std::ptr;
use super::math;

use platform::glfw::{Context};

pub mod primitives;
mod opengl;

#[derive(Debug)]
struct RenderBuffer {
    data : Vec<f32>,
    vbo_id : u32,
    shader_id : u32,
}

#[derive(Debug)]
pub struct Renderer {
    buffers : Vec<RenderBuffer>,
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
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
        }
        let mut renderer = Renderer { buffers: Vec::new(), vao_id: vao};
        println!("New renderer for {:?}", window);
        renderer.new_render_buffer(program);
        renderer
    }

    pub fn render(&mut self, window : &mut glfw::Window) {

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let window_size = window.get_size();

        for buffer in &mut self.buffers {
            Renderer::render_buffer(buffer, window_size);
            buffer.data.clear();
        }

        window.swap_buffers();
    }


    fn render_buffer(buffer: &mut RenderBuffer, window_size : (i32, i32)) {

        let out_color_str = CString::new("out_color").unwrap();
        let position_str = CString::new("position").unwrap();
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

            // Specify layout
            let pos_attr = gl::GetAttribLocation(buffer.shader_id, position_str.as_ptr());
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                2 * mem::size_of::<GLfloat>() as i32,
                ptr::null(),
            );

            let mvp = math::array4x4(math::ortho(0.0, window_size.0 as f32, window_size.1 as f32, 0.0, 0.0, 1.0));

            let mvp_attr = gl::GetUniformLocation(buffer.shader_id, mvp_str.as_ptr());
            gl::UniformMatrix4fv(mvp_attr, 1, gl::FALSE, mvp.as_ptr() as *const f32);

            gl::DrawArrays(gl::TRIANGLES, 0, buffer.data.len() as i32);
        }
    }

    fn new_render_buffer(&mut self, shader_id : u32) {

        let mut vbo = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }
        self.buffers.push( RenderBuffer { data: Vec::new(), vbo_id : vbo, shader_id : shader_id });
    }

    fn add_to_buffer(&mut self, data: &mut Vec<f32>) {
        let buff : &mut RenderBuffer = self.buffers.first_mut().unwrap();
        buff.data.append(data);
    }
}

pub fn resize(width : i32, height : i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
    println!("Resize to {},{}", width, height);
}
