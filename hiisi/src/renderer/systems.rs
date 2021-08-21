use crate::{
    assets::TextureAssets,
    components::*,
    ecs::{Read, ReadExpect, ReadStorage, System, SystemData, World, Write},
    platform::window::WindowSize,
};

pub use super::{components::*, QuadBuffer, RenderBatch, Renderer};

pub struct RenderSystem {
    renderer: Renderer,
    vbo_id: u32,
    shader_id: u32,
}

impl RenderSystem {
    pub fn new() -> Self {

        let renderer = Renderer::new();
        let (vbo_id, shader_id) = renderer.get_quad_buffer_vbo_and_shader();

        RenderSystem {
            renderer,
            vbo_id,
            shader_id,
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Write<'a, QuadBuffer>, // Write for clearing
        Read<'a, WindowSize>,
    );

    fn run(&mut self, (mut quads, window_size): Self::SystemData) {
        self.renderer.render_quad_buffer(
            &mut *quads,
            self.vbo_id,
            self.shader_id,
            (window_size.0, window_size.1),
        );
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }
}

pub struct SpriteSystem;
impl<'a> System<'a> for SpriteSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Texture>,
        ReadStorage<'a, Color>,
        ReadExpect<'a, TextureAssets>,
        Write<'a, QuadBuffer>,
    );

    fn run(&mut self, (poss, sizes, txtrs, cols, textures, mut buffer): Self::SystemData) {
        use ecs::Join;
        for (pos, size, txtr, col) in (&poss, &sizes, &txtrs, &cols).join() {
            Renderer::add_quad_to_buffer(
                &mut buffer,
                pos.0,
                size.0,
                *col,
                textures
                    .get_render_id(
                        textures
                            .get_texture_id(&txtr.0)
                            .expect("Couldn't find texture"),
                    )
                    .expect("Couldn't find render_id"),
            );
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
    }
}
