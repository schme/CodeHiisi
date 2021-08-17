extern crate glfw;

pub use self::glfw::WindowEvent;
use platform::{MouseButtonState, WindowSize};

use ecs::{System, SystemData, Write, World};

use self::glfw::{
    Window,
    Context,
};

pub struct WindowSystem {
    window: Window,
}

impl WindowSystem {
    pub fn new(window: Window) -> Self {
        WindowSystem { window }
    }
}

impl<'a> System<'a> for WindowSystem {
    type SystemData =
        (Write<'a, MouseButtonState>,
        Write<'a, WindowSize>);

    fn run(&mut self, (mut mbs, mut window_size): Self::SystemData) {
        self.window.glfw.poll_events();

        let (width, height) = self.window.get_framebuffer_size();
        *window_size = WindowSize(width, height);

        self.window.swap_buffers();
    }


    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        let window = &mut self.window;

        window.set_key_polling(true);
        window.set_close_polling(true);
        window.set_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);

        {
            use renderer;
            renderer::get_proc_address(window);
        }

        window.make_current();
        window.glfw.set_swap_interval(glfw::SwapInterval::Sync(0));
    }
}
