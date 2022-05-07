mod align;
mod button;
mod element;

use ::anyhow::Result;

use crate::{
    immediate_mode_graphics::{Drawable, Frame},
    ui2::{
        primitives::{Rect, Tile},
        Dimensions, Id, Input, InternalState,
    },
    vec2, vec4, Vec2, Vec4,
};

pub use self::{
    align::{Align, HAlignment, VAlignment},
    button::Button,
    element::Element,
};

pub trait Widget<Message> {
    /// Handle events for this widget.
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<Message>>;

    /// Render this widget to the current frame.
    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()>;

    /// Compute the widget's dimensions.
    /// A widget must not allow its dimensions to exceed the provided max
    /// size.
    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions;

    /// Set this widget's top-left position in screen space.
    /// This is always called by the parent widget.
    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    );
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Activity {
    Inactive,
    Hover,
    Pressed,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct ContainerState {
    pub activity: Activity,
}

impl Default for ContainerState {
    fn default() -> Self {
        Self {
            activity: Activity::Inactive,
        }
    }
}

pub struct Container<Message> {
    id: Id,
    dimensions: Dimensions,
    original_dimensions: Dimensions,
    position: Vec2,
    rgba: Vec4,
    hover_color: Vec4,
    pressed_color: Vec4,
    should_grow: bool,
    on_click: Option<Message>,
}

impl<Message> Container<Message> {
    pub fn new(id: Id, width: f32, height: f32) -> Self {
        Self {
            id,
            dimensions: (width, height).into(),
            original_dimensions: (width, height).into(),
            position: vec2(0.0, 0.0),
            rgba: vec4(1.0, 1.0, 1.0, 1.0),
            hover_color: vec4(1.0, 0.0, 0.0, 1.0),
            pressed_color: vec4(0.0, 0.0, 0.0, 1.0),
            should_grow: true,
            on_click: None,
        }
    }

    pub fn with_color(self, rgba: Vec4) -> Self {
        Self { rgba, ..self }
    }

    pub fn with_should_grow(self, should_grow: bool) -> Self {
        Self {
            should_grow,
            ..self
        }
    }

    pub fn with_on_click(self, message: Message) -> Self {
        Self {
            on_click: Some(message),
            ..self
        }
    }

    fn bounds(&self) -> Rect {
        Rect::new(
            self.position.y,
            self.position.x,
            self.position.y + self.dimensions.height,
            self.position.x + self.dimensions.width,
        )
    }
}

impl<Message> Widget<Message> for Container<Message>
where
    Message: Copy,
{
    /// Handle events for this widget.
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<Message>> {
        use glfw::{Action, MouseButton, WindowEvent};

        let state = internal_state.get_state_mut::<ContainerState>(&self.id);
        let message = match *event {
            WindowEvent::CursorPos(x, y) => {
                if self.bounds().contains(vec2(x as f32, y as f32)) {
                    if state.activity == Activity::Inactive {
                        state.activity = Activity::Hover;
                    }
                } else {
                    state.activity = Activity::Inactive;
                }
                None
            }
            WindowEvent::MouseButton(
                MouseButton::Button1,
                Action::Press,
                _,
            ) => {
                if state.activity == Activity::Hover {
                    log::info!("pressed");
                    state.activity = Activity::Pressed;
                }
                None
            }
            WindowEvent::MouseButton(
                MouseButton::Button1,
                Action::Release,
                _,
            ) => {
                if state.activity == Activity::Pressed {
                    if self.bounds().contains(input.mouse_position) {
                        state.activity = Activity::Hover;
                    } else {
                        state.activity = Activity::Inactive;
                    }
                    // this button was active, therefore this is a 'click'
                    self.on_click
                } else {
                    // the release was unrelated to the button
                    None
                }
            }
            _ => None,
        };
        Ok(message)
    }

    /// Render this widget to the current frame.
    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()> {
        let state = internal_state.get_state::<ContainerState>(&self.id);
        let color = match state.activity {
            Activity::Inactive => self.rgba,
            Activity::Hover => self.hover_color,
            Activity::Pressed => self.pressed_color,
        };
        Tile {
            model: Rect::new(
                self.position.y,
                self.position.x,
                self.position.y + self.dimensions.height,
                self.position.x + self.dimensions.width,
            ),
            color,
            ..Default::default()
        }
        .fill(frame)
    }

    /// Containers grow to their maximum size by default.
    fn dimensions(
        &mut self,
        _internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        if self.should_grow {
            self.dimensions = *max_size;
        } else {
            self.dimensions = max_size.min(&self.original_dimensions);
        }
        self.dimensions
    }

    /// Set the container's top left position.
    fn set_top_left_position(
        &mut self,
        _internal_state: &mut InternalState,
        position: Vec2,
    ) {
        self.position = position;
    }
}
