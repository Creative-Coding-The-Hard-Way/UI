use crate::{vec2, Vec2};

use ::anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Id {
    Number(i32),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseState {
    Pressed,
    NotPressed,
}

#[derive(Debug, Copy, Clone)]
pub struct State {
    mouse_position: Vec2,
    mouse_state: MouseState,
    active_item: Id,
    hot_item: Id,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mouse_position: vec2(0.0, 0.0),
            mouse_state: MouseState::NotPressed,
            active_item: Id::Number(0),
            hot_item: Id::Number(0),
        }
    }
}

impl State {
    /// Create a new UI state instance.
    pub fn new() -> Self {
        Default::default()
    }

    /// Get the current mouse position.
    pub fn get_mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    /// Get the mouse's current state.
    pub fn get_mouse_state(&self) -> MouseState {
        self.mouse_state
    }

    /// Handle window events and update internal data structures.
    pub fn handle_event(&mut self, event: glfw::WindowEvent) -> Result<()> {
        use glfw::{Action, MouseButton, WindowEvent};

        match event {
            WindowEvent::CursorPos(x, y) => {
                self.mouse_position = vec2(x as f32, y as f32);
            }
            WindowEvent::MouseButton(
                MouseButton::Button1,
                Action::Press,
                _,
            ) => {
                self.mouse_state = MouseState::Pressed;
            }
            WindowEvent::MouseButton(
                MouseButton::Button1,
                Action::Release,
                _,
            ) => {
                self.mouse_state = MouseState::NotPressed;
            }
            _ => (),
        }
        Ok(())
    }
}
