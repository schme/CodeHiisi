use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub fn file_to_str<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    Ok(contents)
}


pub mod image {

    extern crate stb_image;

    pub use self::stb_image::image::{Image, LoadResult};
    pub use self::stb_image::image::{load};

    //use self::stb_image::image::*;

    pub fn setup() {
        unsafe {
            self::stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
        }
    }
}
