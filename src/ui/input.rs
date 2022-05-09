use crate::{vec2, Vec2};

/// This struct holds all persistent UI input state. Things like the position
/// of the mouse or hotkeys that are still held down.
#[derive(Debug, Copy, Clone)]
pub struct Input {
    pub mouse_position: Vec2,
}

impl Input {
    /// Create a new instance with default values.
    pub fn new() -> Self {
        Self {
            mouse_position: vec2(0.0, 0.0),
        }
    }

    /// Handle system events to update internal state.
    pub fn handle_event(&mut self, event: &glfw::WindowEvent) {
        match *event {
            glfw::WindowEvent::CursorPos(x, y) => {
                self.mouse_position = vec2(x as f32, y as f32);
            }
            _ => (),
        }
    }
}
