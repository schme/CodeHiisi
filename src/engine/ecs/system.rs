use ecs::{
    data::DynamicData,
    world::World,
};

pub trait System<'a> {
    type SystemData: DynamicData<'a>;

    fn run_now(&mut self, world: World) {
        self.run(Self::SystemData::fetch(world));
    }

    fn run(&mut self, data: Self::SystemData);
}

