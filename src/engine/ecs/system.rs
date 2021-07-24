use ecs::{
    data::DynamicData,
    world::World,
};

pub trait System<'a> {
    type SystemData: DynamicData<'a>;

    fn run_now(&mut self, world: &'a World) {
        let data = *world.get::<Self::SystemData>();
        //self.run(data);
    }

    fn run(&mut self, data: Self::SystemData);
}

