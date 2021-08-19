extern crate glfw;

pub use self::glfw::WindowEvent;
use platform::{MouseButtonState, WindowSize};

use ecs::{System, SystemData, World, Write};

use self::glfw::{Context, Window};

pub struct WindowSystem {
    window: Window,
}

impl WindowSystem {
    pub fn new(mut window: Window) -> Self {

        window.set_key_polling(true);
        window.set_close_polling(true);
        window.set_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);

        {
            use renderer;
            renderer::get_proc_address(&mut window);
        }

        window.make_current();
        window.glfw.set_swap_interval(glfw::SwapInterval::Sync(1));
        WindowSystem { window }
    }
}

impl<'a> System<'a> for WindowSystem {
    type SystemData = (Write<'a, MouseButtonState>, Write<'a, WindowSize>);

    fn run(&mut self, (mut mbs, mut window_size): Self::SystemData) {
        log::trace!("Running WindowSystem");
        self.window.glfw.poll_events();

        let (width, height) = self.window.get_framebuffer_size();
        *window_size = WindowSize(width, height);

        *mbs = MouseButtonState {
            left: self.window.get_mouse_button(glfw::MouseButtonLeft),
            right: self.window.get_mouse_button(glfw::MouseButtonRight),
            middle: self.window.get_mouse_button(glfw::MouseButtonMiddle),
        };

        {
            use std::time::Instant;
            let swap_timer = Instant::now();

            self.window.swap_buffers();
            log::trace!("Time spent waiting swap: {:?}", swap_timer.elapsed());
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

    }
}
