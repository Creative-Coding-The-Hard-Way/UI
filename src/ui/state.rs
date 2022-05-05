use crate::{
    immediate_mode_graphics::{Drawable, Frame},
    math,
    ui::primitives::{Rect, Tile},
    vec2, vec4, Mat4, Vec2,
};

use ::anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Id {
    Number(i32),
}

/// Generate a simple hash from a given string at compile time.
pub const fn id_hash(content: &str, line: u32, column: u32, seed: u32) -> u32 {
    let content_bytes = content.as_bytes();
    let mut hash = 3581u32;
    let mut i: usize = 0;
    while i < content_bytes.len() {
        hash = hash.wrapping_mul(33).wrapping_add(content_bytes[i] as u32);
        i += 1;
    }
    hash = hash.wrapping_mul(33).wrapping_add(line);
    hash = hash.wrapping_mul(33).wrapping_add(column);
    hash = hash.wrapping_mul(33).wrapping_add(seed);
    return hash;
}

#[macro_export]
macro_rules! gen_id {
    () => {{
        const ID: u32 = ccthw::ui::id_hash(file!(), line!(), column!(), 17);
        Id::Number(ID as i32)
    }};
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
    screen_dimensions: Vec2,

    /// A projection matrix for the UI which defines a coordinate system
    /// starting at (0,0) in the top left of the screen and (width, height) in
    /// the bottom right of the screen.
    projection: Mat4,

    active_item: Id,
    hot_item: Id,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mouse_position: vec2(0.0, 0.0),
            mouse_state: MouseState::NotPressed,
            screen_dimensions: vec2(1.0, 1.0),
            projection: Mat4::identity(),
            active_item: Id::Number(0),
            hot_item: Id::Number(0),
        }
    }
}

impl State {
    /// Create a new UI state instance.
    pub fn new(screen_width: i32, screen_height: i32) -> Self {
        Self {
            screen_dimensions: vec2(screen_width as f32, screen_height as f32),
            projection: Self::ortho_projection(
                screen_width as f32,
                screen_height as f32,
            ),
            ..Default::default()
        }
    }

    /// Get the current mouse position.
    pub fn get_mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    /// Get the mouse's current state.
    pub fn get_mouse_state(&self) -> MouseState {
        self.mouse_state
    }

    /// Get a view projection matrix which defines coordinate values where
    /// (0,0) is the top left corner of the screen and (width,height) is the
    /// bottom right corner of the screen.
    ///
    /// E.g. positive X points to the right, and positive Y points down.
    ///
    /// The z-axis ranges from 0.0 on the near plane and 1.0 on the far plane,
    /// but most of the time depth-testing is disabled for UI rendering so this
    /// is typically unimportant.
    pub fn get_projection(&self) -> Mat4 {
        self.projection
    }

    pub fn button(
        &mut self,
        frame: &mut Frame,
        id: Id,
        pos: Vec2,
    ) -> Result<bool> {
        let region = Rect::new(pos.y, pos.x, pos.y + 128.0, pos.x + 256.0);
        if region.contains(self.get_mouse_position()) {
            self.hot_item = id; // on hover
            if self.active_item == Id::Number(0)
                && self.get_mouse_state() == MouseState::Pressed
            {
                self.active_item = id; // on click
            }
        }

        // render drop shadow
        Tile {
            model: region.translate(vec2(15.0, 15.0)),
            color: vec4(0.0, 0.0, 0.0, 0.2),
            ..Default::default()
        }
        .fill(frame)?;

        if self.hot_item == id {
            if self.active_item == id {
                Tile {
                    model: region.translate(vec2(5.0, 5.0)),
                    ..Default::default()
                }
                .fill(frame)?;
            } else {
                Tile {
                    model: region,
                    ..Default::default()
                }
                .fill(frame)?;
            }
        } else {
            Tile {
                model: region,
                color: vec4(0.5, 0.5, 0.5, 1.0),
                ..Default::default()
            }
            .fill(frame)?;
        }

        Ok(self.get_mouse_state() == MouseState::NotPressed
            && self.hot_item == id
            && self.active_item == id)
    }

    pub fn prepare(&mut self) {
        self.hot_item = Id::Number(0);
    }

    pub fn finish(&mut self) {
        if self.get_mouse_state() == MouseState::NotPressed {
            self.active_item = Id::Number(0);
        } else {
            if self.active_item == Id::Number(0) {
                self.active_item = Id::Number(-1);
            }
        }
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
            WindowEvent::FramebufferSize(width, height) => {
                self.screen_dimensions = vec2(width as f32, height as f32);
                self.projection =
                    Self::ortho_projection(width as f32, height as f32);
            }
            _ => (),
        }
        Ok(())
    }
}

impl State {
    fn ortho_projection(width: f32, height: f32) -> Mat4 {
        math::projections::ortho(0.0, width, height, 0.0, 0.0, 1.0)
    }
}
