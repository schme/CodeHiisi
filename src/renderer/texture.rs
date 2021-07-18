use std::path::Path;
use std::collections::HashMap;
use std::{fs, io};
use super::gl::types::*;

use crate::platform::file::image::{self, LoadResult};
use crate::renderer;

pub struct TextureStorage {
    pub data: HashMap<String, Texture>,
}

pub struct Texture {
    pub id: u32,
    pub image: image::Image<u8>,
}

impl TextureStorage {
    pub fn new() -> TextureStorage {

        TextureStorage {
            data: HashMap::new(),
        }
    }

    pub fn get_texture(&self, texture_name: &str) -> Option<&Texture> {
        self.data.get(texture_name)
    }

    pub fn new_texture(&mut self, path: &Path) {

        let texture_name = String::from(path.file_name().unwrap().to_str().unwrap());

        let txtr = image::load(path);
        match txtr {
            LoadResult::ImageU8(img) =>  {
                println!("ImageU8 found at {:?}:", path);
                println!("{}, {}, {}", img.width, img.height, img.depth);
                let texture = Texture {
                    id: renderer::gen_texture(),
                    image: img,
                };
                unsafe {
                    gl::BindTexture(gl::TEXTURE_2D, texture.id);

                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); 
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32); 
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32); 
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32); 

                    gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32,
                        texture.image.width as i32, texture.image.height as i32, 0, gl::RGBA,
                        gl::UNSIGNED_BYTE, texture.image.data.as_ptr() as *const GLvoid);
                }
                self.data.insert(texture_name, texture);
            },
            LoadResult::ImageF32(img) => {
                println!("ImageF32 found at {:?}:", path);
                println!("{}, {}, {}", img.width, img.height, img.depth);
            },
            LoadResult::Error(s) => {
                println!("Failed to load image at {:?}: {}", path, s);
            },
        }
    }

    pub fn load_textures<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.load_textures(path);
            }
            else {
                self.new_texture(&path);
            }
        }

        Ok(())
    }
}
