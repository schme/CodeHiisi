extern crate gl;

use self::gl::types::*;

use std::ffi::CString;
use std::ptr;
use std::str;

static VS_SRC: &'static str = "
#version 410 core
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 texcoord;

out vec4 vertex_color;
out vec2 tex_coord;

uniform mat4 mvp;

void main() {
    gl_Position = mvp * vec4(position, 0.0f, 1.0f);
    vertex_color = vec4(color, 1.0f);
    tex_coord = texcoord;
}";


static FS_SRC: &'static str = "
#version 410 core

in vec4 vertex_color;
in vec2 tex_coord;
out vec4 out_color;

uniform sampler2D a_texture;

void main() {
    out_color = texture(a_texture, tex_coord) * vertex_color;
}";


pub fn make_temp_shader() ->GLuint {
    make_shader(VS_SRC, FS_SRC)
}

pub fn make_shader(vert: &str, frag: &str) -> GLuint {
    let vs = compile_shader(vert, gl::VERTEX_SHADER);
    let fs = compile_shader(frag, gl::FRAGMENT_SHADER);

    link_program(vs, fs)
}


fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);

        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                .ok()
                .expect("ShaderInfoLog not valid utf8")
            );
        }
    }
    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);

        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}

pub fn bind_texture_by_id(texture_id: u32) {
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

pub fn resize_viewport(width : i32, height : i32) {
    unsafe {
        gl::Viewport(0, 0, width, height);
    }
    println!("Resize to {},{}", width, height);
}
