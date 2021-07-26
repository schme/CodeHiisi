use ecs::{
    data::{SystemData},
    world::World,
};

pub trait System<'a> {
    type SystemData: SystemData<'a>;

    fn run_now(&mut self, world: &'a World) {
        let data: Self::SystemData = SystemData::fetch(world);
        self.run(data);
    }

    fn setup(world: &mut World) {
        <Self::SystemData as SystemData<'a>>::setup(world);
    }

    fn run(&mut self, data: Self::SystemData);
}
