use ecs::{
    system::{System, SystemData},
}

pub struct Dispatcher<'a> {
    systems: Vec<Box<dyn System>>,
}

impl Dispatcher<'a> {
}
