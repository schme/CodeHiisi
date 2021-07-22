use {
    std::{
        fs, io,
        path::Path,
        collections::HashMap,
    },
    renderer::gl::types::*,
    platform::file::image::{
        self, LoadResult
    },
    renderer,
};


pub struct TextureStorage {
    pub data: HashMap<String, Texture>,
}

pub struct Texture {
    pub id: u32,
    pub image: image::Image<u8>,
}

impl TextureStorage {

    pub fn new() -> Self {
        TextureStorage {
            data: HashMap::new(),
        }
    }

    pub fn get_white_id(&self) -> u32 {
        self.data.get("white.png").unwrap().id
    }

    pub fn get_texture_id(&self, texture_name: &str) -> Option<u32> {
        if let Some(texture) = self.data.get(texture_name) {
            return Some(texture.id);
        }
        None
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
                println!("name: {}, id: {}", texture_name, texture.id);
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
                self.data.insert(texture_name, texture);
            },
            LoadResult::ImageF32(img) => {
                println!("ImageF32 found at {:?}:", path);
                println!("{}, {}, {}", img.width, img.height, img.depth);
                println!("Discarded!");
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
                match self.load_textures(path) {
                    Err(s) => println!("Failed to load textures: {}", s),
                    _ => {},
                }
            }
            else {
                self.new_texture(&path);
            }
        }

        Ok(())
    }
}
