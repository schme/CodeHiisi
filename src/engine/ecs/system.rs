use ecs::{
    data::{SystemData, DynamicData},
    world::World,
};

pub trait System<'a> {
    type SystemData: SystemData<'a>;

    fn run_now(&mut self, world: &'a World) {
        let data: Self::SystemData = SystemData::fetch(world);
        self.run(data);
    }

    fn run(&mut self, data: Self::SystemData);
}

