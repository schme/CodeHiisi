use super::*;

use crate::{
    ecs::{Component, DenseVecStorage},
};


#[derive(Debug)]
#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Cuboid(shape::Cuboid);
