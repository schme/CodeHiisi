extern crate stb_image;

pub use self::stb_image::image::{Image, LoadResult};
pub use self::stb_image::image::{load};

//use self::stb_image::image::*;

pub fn setup() {
    unsafe {
        self::stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
    }
}
