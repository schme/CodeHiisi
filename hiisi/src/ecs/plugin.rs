use ecs::{World, System, DispatcherBuilder};

pub trait Plugin {

    fn load(&mut self, world: &mut World, dispatcher: &mut DispatcherBuilder);

    fn unload(&mut self) {
    }
}
