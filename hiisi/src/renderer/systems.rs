use crate::{
    assets::TextureAssets,
    components::*,
    ecs::{Read, ReadExpect, ReadStorage, System, SystemData, World, Write},
    platform::WindowSize,
};

pub use super::{components::*, QuadBuffer, RenderBatch, Renderer};

pub struct RenderSystem {
    renderer: Option<Renderer>,
    vbo_id: u32,
    shader_id: u32,
}

impl RenderSystem {
    pub fn new() -> Self {
        RenderSystem {
            renderer: None,
            vbo_id: 0,
            shader_id: 0,
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Write<'a, QuadBuffer>, // Write for clearing
        Read<'a, WindowSize>,
    );

    fn run(&mut self, (mut quads, window_size): Self::SystemData) {
        log::trace!("Running RenderSystem");
        if let Some(rend) = &mut self.renderer {
            rend.render_quad_buffer(
                &mut *quads,
                self.vbo_id,
                self.shader_id,
                (window_size.0, window_size.1),
            );
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let rend = Renderer::new();
        let (vbo_id, shader_id) = rend.get_quad_buffer_vbo_and_shader();

        let mut textures = world.fetch_mut::<TextureAssets>();
        textures.gen_loaded_textures();

        self.vbo_id = vbo_id;
        self.shader_id = shader_id;

        self.renderer = Some(rend);

        textures.push_loaded_textures();
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
        log::trace!("Running SpriteSystem");
        use ecs::Join;
        for (pos, size, txtr, col) in (&poss, &sizes, &txtrs, &cols).join() {
            Renderer::add_quad_to_buffer(
                &mut buffer,
                pos.0,
                size.0,
                col.0,
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
