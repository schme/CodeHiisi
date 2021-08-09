use {
    renderer::gl::types::*,
    assets::{Texture},
};

pub fn load_texture(texture: &Texture) {
    unsafe {
        gl::BindTexture(gl::TEXTURE_2D, texture.id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); 
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32); 
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32); 
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); 

        let mode = if texture.image.depth == 4 { gl::RGBA } else { gl::RGB };
        gl::TexImage2D(gl::TEXTURE_2D, 0, mode as i32,
            texture.image.width as i32, texture.image.height as i32, 0, mode,
            gl::UNSIGNED_BYTE, texture.image.data.as_ptr() as *const GLvoid);

        gl::BindTexture(gl::TEXTURE_2D, 0);
    }
}

pub fn gen_texture() -> u32 {
    let mut texture_id = 0;
    unsafe {
        gl::GenTextures(1, &mut texture_id);
    }
    texture_id
}

