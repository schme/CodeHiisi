use {
    std::{
        fs, io,
        path::Path,
        collections::HashMap,
    },
    platform::file::image::{
        self, LoadResult
    },
    renderer,
};

pub type TextureId = u32;

pub struct TextureStorage {
    pub data: HashMap<String, Texture>,
}

pub struct Texture {
    pub id: TextureId,
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

    pub fn push_loaded_textures(&self) {
        for (_, texture) in &self.data {
            renderer::load_texture(&texture);
        }
    }

    pub fn load_textures_from_path<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                match self.load_textures_from_path(path) {
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
