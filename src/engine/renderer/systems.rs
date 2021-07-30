use engine:: {
    platform::{RenderContext, Context, WindowSize},
    components::*,
    ecs::{World, System, SystemData, Read, Write, ReadStorage},
};

pub use super::{
    Renderer,
    components::*,
    RenderBatch, QuadBuffer,
};


pub struct RenderSystem {
    renderer: Option<Renderer>,
    context: RenderContext,
    vbo_id: u32,
    shader_id: u32,
}

impl RenderSystem {
    pub fn new(context: RenderContext) -> Self {
        let renderer = Renderer::new();
        let (vbo_id, shader_id) = renderer.get_quad_buffer_vbo_and_shader();

        RenderSystem {
            renderer: Some(renderer),
            context,
            vbo_id,
            shader_id,
        }
    }
}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (Write<'a, QuadBuffer>, // Write for clearing
                        Read<'a, WindowSize>);

    fn run(&mut self, (mut quads, window_size): Self::SystemData) {
        if let Some(renderer) = &mut self.renderer {
            renderer.render_quad_buffer( 
                &mut *quads, self.vbo_id, self.shader_id,
                (window_size.0, window_size.1), &mut self.context);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
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
}
