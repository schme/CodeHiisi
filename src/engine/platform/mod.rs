pub extern crate glfw;

pub mod file;

use std::str;
use engine::renderer;
pub use self::glfw::{Action, Key};

pub struct Platform
{
    pub window: glfw::Window,
    pub events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,

    should_close : bool,

    cursor_x: f64,
    cursor_y: f64,
}

pub struct MouseButtonState {
    pub left: Action,
    pub right: Action,
    pub middle: Action,
}

impl Platform {

    pub fn new(title: &str, width: u32, height: u32) -> Platform {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));

        let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW Window");

        window.set_key_polling(true);
        window.set_close_polling(true);
        window.set_size_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);

        file::image::setup();

        Platform { window, events, should_close: false, cursor_x: 0.0, cursor_y: 0.0 }
    }

    pub fn poll_events(&mut self) {
        self.window.glfw.poll_events();
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn handle_events(&mut self) {
        // Poll for and process events
        for (_, event) in glfw::flush_messages(&self.events) {
            //println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _)
                    | glfw::WindowEvent::Close => {
                    self.window.set_should_close(true);
                    self.should_close = true;
                },
                glfw::WindowEvent::Size(window_width, window_height) => {
                    self.resize(window_width, window_height);
                },
                glfw::WindowEvent::CursorPos(cursor_x, cursor_y) => {
                    self.cursor_x = cursor_x;
                    self.cursor_y = cursor_y;
                }
                _ => {},
            }
        }
    }

    pub fn get_time(&self) -> f64 {
        self.window.glfw.get_time()
    }

    pub fn get_cursor_pos(&self) -> (f64, f64) {
        (self.cursor_x, self.cursor_y)
    }

    pub fn get_mouse_state(&self) -> MouseButtonState {
        let left = self.window.get_mouse_button(glfw::MouseButtonLeft);
        let right = self.window.get_mouse_button(glfw::MouseButtonRight);
        let middle = self.window.get_mouse_button(glfw::MouseButtonMiddle);
        MouseButtonState{ left, right, middle }
    }

    fn resize(&self, _window_width : i32, _window_height : i32) {
        let (width, height) = self.window.get_framebuffer_size();
        renderer::resize_viewport(width, height);
    }
}

