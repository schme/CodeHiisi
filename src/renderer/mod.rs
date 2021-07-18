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
use platform::file::image::Image;
use self::texture::{Texture, TextureStorage};

mod opengl;

#[derive(Debug)]
struct RenderBuffer {
    data : Vec<f32>,
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
        let mut txtr_id = 0;

        let program = opengl::make_temp_shader();

        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        }
        let mut renderer = Renderer { buffers: Vec::new(), textures: TextureStorage::new(), vao_id: vao};
        println!("New renderer for {:?}", window);
        renderer.new_render_buffer(program);
        renderer
    }

    pub fn render(&mut self, window : &mut glfw::Window) {

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
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
        let color_str = CString::new("color").unwrap();
        let texcoord_str = CString::new("texcoord").unwrap();
        let txtr_str = CString::new("a_texture").unwrap();
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
            let col_attr = gl::GetAttribLocation(buffer.shader_id, color_str.as_ptr());
            let tc_attr = gl::GetAttribLocation(buffer.shader_id, texcoord_str.as_ptr());

            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                7 * mem::size_of::<GLfloat>() as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                col_attr as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                7 * mem::size_of::<GLfloat>() as i32,
                (2 * mem::size_of::<GLfloat>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(col_attr as GLuint);
            gl::VertexAttribPointer(
                tc_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                7 * mem::size_of::<GLfloat>() as i32,
                (5 * mem::size_of::<GLfloat>()) as *const GLvoid,
            );
            gl::EnableVertexAttribArray(tc_attr as GLuint);

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

    pub fn load_textures<P: AsRef<Path>>(&mut self, path: P) {
        self.textures.load_textures(path);
    }

    pub fn use_texture(&mut self, texture_name: &str) {
        if let Some(texture) = self.textures.get_texture(texture_name) {
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, texture.id);
            }
        }
    }

    pub fn add_quad(&mut self, position: Point2<f32>, size: Vector2<f32>, color: Vector3<f32>) {
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
        self.add_to_buffer(&mut v);
    }

}

pub fn use_texture_by_id(texture_id: u32) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
    }
}

pub fn gen_texture() -> u32 {
    let mut texture_id = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
    }
    texture_id
}

pub fn resize(width : i32, height : i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
    println!("Resize to {},{}", width, height);
}
