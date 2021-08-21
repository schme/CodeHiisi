extern crate glfw;

pub use self::glfw::WindowEvent;

use input::pointer::MouseButtonState;

use ecs::{System, SystemData, World, Write};

use self::glfw::{Context, Window};
use platform::events::Action;

#[derive(Default)]
pub struct WindowSize(pub i32, pub i32);


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
        self.window.glfw.poll_events();

        let (width, height) = self.window.get_framebuffer_size();
        *window_size = WindowSize(width, height);

        *mbs = MouseButtonState {
            left: Action::from_platform_action(self.window.get_mouse_button(glfw::MouseButtonLeft)),
            right: Action::from_platform_action(self.window.get_mouse_button(glfw::MouseButtonRight)),
            middle: Action::from_platform_action(self.window.get_mouse_button(glfw::MouseButtonMiddle)),
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
