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
pub type RenderId = u32;

pub struct TextureAssets {
    name_map: HashMap<String, TextureId>,
    data: Vec<Texture>,
    texture_path: String,
}

pub struct Texture {
    pub render_id: Option<RenderId>,
    pub image: image::Image<u8>,
}

impl TextureAssets {

    pub fn new(texture_path: String) -> Self {
        let mut assets = TextureAssets {
            name_map: HashMap::new(),
            data: Vec::new(),
            texture_path,
        };
        assets.load_textures();
        assets
    }

    pub fn get_white_id(&self) -> TextureId {
        *self.name_map.get("white.png").unwrap()
    }

    pub fn get_texture_id(&self, texture_name: &str) -> Option<TextureId> {
        if let Some(id) = self.name_map.get(texture_name) {
            return Some(*id);
        }
        None
    }

    pub fn get_render_id(&self, texture_id: TextureId) -> Option<RenderId> {
        self.data[texture_id as usize].render_id
    }

    pub fn new_texture(&mut self, path: &Path) {
        let texture_name = String::from(path.file_name().unwrap().to_str().unwrap());

        let txtr = image::load(path);
        match txtr {
            LoadResult::ImageU8(img) =>  {
                println!("ImageU8 found at {:?}:", path);
                println!("{}, {}, {}", img.width, img.height, img.depth);
                let texture = Texture {
                    render_id: None,
                    image: img,
                };
                let id: TextureId = self.data.len() as u32;
                println!("name: {}, id: {}", texture_name, id);

                self.data.push(texture);
                self.name_map.insert(texture_name, id);

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

    pub fn gen_loaded_textures(&mut self) {
        for mut texture in self.data.iter_mut() {
            texture.render_id = Some(renderer::gen_texture());
        }
    }

    pub fn push_loaded_textures(&self) {
        for texture in self.data.iter() {
            renderer::load_texture(&texture);
        }
    }

    pub fn load_textures(&mut self) -> io::Result<()> {
        let txtr_path = self.texture_path.clone();
        self.load_textures_from_path(&txtr_path)
    }

    fn load_textures_from_path<P: AsRef<Path>>(&mut self, path: &P) -> io::Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                if let Err(s) = self.load_textures_from_path(&path) {
                    println!("Failed to load textures: {}", s)
                }
            }
            else {
                self.new_texture(&path);
            }
        }

        Ok(())
    }
}
