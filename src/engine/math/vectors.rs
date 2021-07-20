use std::{
    ops::{Add},
    marker::{Copy},
};

#[derive(Debug)]
pub struct V2<T: Add> {
    pub x: T,
    pub y: T,
}

impl<T: Copy + Add + Add<Output =T>> Add for V2<T> {
    type Output = V2<T>;

    #[allow(dead_code)]
    fn add(self, other: V2<T>) -> V2<T> {
        V2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Add> V2<T> {

    #[allow(dead_code)]
    pub fn new(x: T, y: T) -> V2<T> {
        V2{x, y}
    }
}

impl<T: Add + Copy> V2<T> {

    #[allow(dead_code)]
    pub fn from_one(v: T) -> V2<T> {
        V2{x: v, y: v}
    }
}


#[derive(Debug)]
pub struct V3<T: Add> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Copy + Add + Add<Output =T>> Add for V3<T> {
    type Output = V3<T>;

    #[allow(dead_code)]
    fn add(self, other: V3<T>) -> V3<T> {
        V3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: Add> V3<T> {
    #[allow(dead_code)]
    pub fn new(x: T, y: T, z: T) -> V3<T> {
        V3{x, y, z}
    }
}

impl<T: Add + Copy> V3<T> {

    #[allow(dead_code)]
    pub fn from_one(v: T) -> V3<T> {
        V3{x: v, y: v, z: v}
    }
}


#[derive(Debug)]
pub struct V4<T: Add> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Copy + Add + Add<Output =T>> Add for V4<T> {
    type Output = V4<T>;

    #[allow(dead_code)]
    fn add(self, other: V4<T>) -> V4<T> {
        V4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<T: Add> V4<T> {
    #[allow(dead_code)]
    pub fn new(x: T, y: T, z: T, w: T) -> V4<T> {
        V4{x, y, z, w}
    }
}

impl<T: Add + Copy> V4<T> {

    #[allow(dead_code)]
    pub fn from_one(v: T) -> V4<T> {
        V4{x: v, y: v, z: v, w: v}
    }
}

