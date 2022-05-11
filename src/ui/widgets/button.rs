use ::anyhow::Result;

use crate::{
    builder_field, builder_field_some,
    immediate_mode_graphics::{Drawable, Frame},
    ui::{
        primitives::{Dimensions, Rect, Tile},
        widgets::{Element, Widget},
        Id, Input, InternalState,
    },
    vec2, vec4, Vec2, Vec4,
};

/// A Button's state is stored in the UI InternalState so it's activity is
/// persisted between views.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ButtonState {
    Inactive,
    Hover,
    Pressed,
}

impl Default for ButtonState {
    /// Defaults to Inactive
    fn default() -> Self {
        Self::Inactive
    }
}

/// A Button is a UI widget which can fire a message when clicked.
pub struct Button<Message> {
    /// The ID uniquely identifies this button when constructing and modifying
    /// state.
    id: Id,

    /// The button's content.
    child: Element<Message>,

    /// The space occupied by the button on screen.
    background: Rect,

    /// The button's default color.
    color: Vec4,

    /// The button's hover color.
    hover_color: Vec4,

    /// The button's color when pressed.
    pressed_color: Vec4,

    /// The message to send when a button click is detected.
    on_click: Option<Message>,
}

impl<Message> Button<Message> {
    pub fn new<W>(id: Id, child: W) -> Self
    where
        W: Into<Element<Message>>,
    {
        Self {
            id,
            child: child.into(),
            background: Rect::new(0.0, 0.0, 0.0, 0.0),
            color: vec4(1.0, 1.0, 1.0, 1.0),
            hover_color: vec4(1.0, 0.0, 0.0, 1.0),
            pressed_color: vec4(0.0, 0.0, 0.0, 1.0),
            on_click: None,
        }
    }

    builder_field!(id, Id);
    builder_field!(color, Vec4);
    builder_field!(hover_color, Vec4);
    builder_field!(pressed_color, Vec4);
    builder_field_some!(on_click, Message);
}

impl<Message> Widget<Message> for Button<Message>
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

        let state = internal_state.get_state_mut::<ButtonState>(&self.id);
        let message = match *event {
            WindowEvent::CursorPos(x, y) => {
                if self.background.contains(vec2(x as f32, y as f32)) {
                    if *state == ButtonState::Inactive {
                        *state = ButtonState::Hover;
                    }
                } else {
                    *state = ButtonState::Inactive;
                }
                None
            }
            WindowEvent::MouseButton(
                MouseButton::Button1,
                Action::Press,
                _,
            ) => {
                if *state == ButtonState::Hover {
                    log::info!("pressed");
                    *state = ButtonState::Pressed;
                }
                None
            }
            WindowEvent::MouseButton(
                MouseButton::Button1,
                Action::Release,
                _,
            ) => {
                if *state == ButtonState::Pressed {
                    if self.background.contains(input.mouse_position) {
                        *state = ButtonState::Hover;
                    } else {
                        *state = ButtonState::Inactive;
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
        let state = internal_state.get_state::<ButtonState>(&self.id);
        let color = match *state {
            ButtonState::Inactive => self.color,
            ButtonState::Hover => self.hover_color,
            ButtonState::Pressed => self.pressed_color,
        };
        Tile {
            model: self.background,
            color,
            ..Default::default()
        }
        .fill(frame)?;

        self.child.draw_frame(internal_state, frame)
    }

    /// Containers grow to their maximum size by default.
    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        let child_dimensions = self.child.dimensions(internal_state, max_size);
        self.background = child_dimensions.as_rect();
        child_dimensions
    }

    /// Set the container's top left position.
    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        let offset = position - self.background.top_left;
        self.background = self.background.translate(offset);
        self.child.set_top_left_position(internal_state, position);
    }
}

impl<Message> Into<Element<Message>> for Button<Message>
where
    Message: 'static + Copy,
{
    fn into(self) -> Element<Message> {
        Element::new(self)
    }
}
