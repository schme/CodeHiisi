use std::ops::{Add};

#[derive(Debug)]
pub struct Rect<T: Add + Copy> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T: Add<Output=T> + Copy> Rect<T> {

    pub fn new(x: T, y: T, w: T, h: T) -> Rect<T> {
        Rect{x, y, w, h}
    }
}
