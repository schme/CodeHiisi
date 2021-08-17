use crate::{
    platform::{RenderContext, WindowSize},
    assets::{TextureAssets},
    components::*,
    ecs::{World, WorldExt, System, SystemData, Read, Write, ReadStorage},
};

pub use super::{
    Renderer,
    components::*,
    RenderBatch, QuadBuffer,
};


pub struct RenderSystem {
    renderer: Option<Renderer>,
    context: Option<RenderContext>,
    vbo_id: u32,
    shader_id: u32,
}

impl RenderSystem {
    pub fn new() -> Self {
        RenderSystem {
            renderer: None,
            context: None,
            vbo_id: 0,
            shader_id: 0,
        }
    }

    pub fn create_renderer(&mut self, context: RenderContext) {
        let renderer = Renderer::new();

        let (vbo_id, shader_id) = renderer.get_quad_buffer_vbo_and_shader();

        self.vbo_id = vbo_id;
        self.shader_id = shader_id;
        self.context = Some(context);
        self.renderer = Some(renderer);
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (Write<'a, QuadBuffer>, // Write for clearing
                        Read<'a, WindowSize>);

    fn run(&mut self, (mut quads, window_size): Self::SystemData) {
        if let Some(renderer) = &mut self.renderer {
            if let Some(context) = &mut self.context {
                renderer.render_quad_buffer( 
                    &mut *quads, self.vbo_id, self.shader_id,
                    (window_size.0, window_size.1), context);
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        //let ctx = *world.read_resource::<RenderContext>();
        //self.create_renderer(ctx);
    }
}

pub struct SpriteSystem;
impl<'a> System<'a> for SpriteSystem {
    type SystemData = 
        (ReadStorage<'a, Position>,
         ReadStorage<'a, Size>,
         ReadStorage<'a, Texture>,
         ReadStorage<'a, Color>,
         Write<'a, QuadBuffer>);

    fn run(&mut self, (poss, sizes, txtrs, cols, mut buffer): Self::SystemData) {
        use ecs::Join;
        for (pos, size, txtr, col) in (&poss, &sizes, &txtrs, &cols).join() {
            Renderer::add_quad_to_buffer(&mut buffer, pos.0, size.0, col.0, txtr.0);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let mut textures = world.fetch_mut::<TextureAssets>();
        textures.gen_loaded_textures();
        textures.push_loaded_textures();
    }
}
