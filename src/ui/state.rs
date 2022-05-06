use crate::{
    immediate_mode_graphics::Frame,
    math,
    ui::{Bounds, Button, Id},
    vec2, Mat4, Vec2,
};

use ::anyhow::Result;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseState {
    Pressed,
    NotPressed,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ActiveItem {
    /// Indicates that no item is currently active.
    None,

    /// The id of the currently active item.
    Some(Id),

    /// No item is active and it's not possible for an item to *become*
    /// active.
    Unavailable,
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

    active_item: ActiveItem,
    hot_item: Option<Id>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mouse_position: vec2(0.0, 0.0),
            mouse_state: MouseState::NotPressed,
            screen_dimensions: vec2(1.0, 1.0),
            projection: Mat4::identity(),
            active_item: ActiveItem::None,
            hot_item: None,
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
        button: Button,
    ) -> Result<bool> {
        if button.bounds().contains(self.get_mouse_position()) {
            self.hot_item = Some(id); // on hover
            if self.active_item == ActiveItem::None
                && self.get_mouse_state() == MouseState::Pressed
            {
                // activate the button when clicked
                self.active_item = ActiveItem::Some(id);
            }
        }

        if self.hot_item == Some(id) {
            if self.active_item == ActiveItem::Some(id) {
                button.draw_active(frame)?;
            } else {
                button.draw_focused(frame)?;
            }
        } else {
            button.draw_unfocused(frame)?;
        }

        let click_complete =
            // A click is complete when the mouse is no longer presesd
            self.get_mouse_state() == MouseState::NotPressed

            // But the mouse is over the current item
            && self.hot_item == Some(id)

            // And the current item *was* active this frame
            && self.active_item == ActiveItem::Some(id);

        Ok(click_complete)
    }

    pub fn render<F>(&mut self, mut func: F) -> Result<()>
    where
        F: FnMut(&mut Self) -> Result<()>,
    {
        self.hot_item = None;
        func(self)?;

        if self.get_mouse_state() == MouseState::NotPressed {
            self.active_item = ActiveItem::None;
        } else {
            if self.active_item == ActiveItem::None {
                self.active_item = ActiveItem::Unavailable;
            }
        }
        Ok(())
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
