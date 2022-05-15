use ::anyhow::Result;

use crate::{
    builder_field, gen_id,
    immediate_mode_graphics::Frame,
    ui::{
        id_hash,
        primitives::Dimensions,
        widgets::{Button, Col, Element, Label, Panel, Widget, WithPadding},
        Font, Id, Input, InternalState,
    },
    vec4, Vec2, Vec4,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WindowState {
    Hidden,
    Visible,
}

impl Default for WindowState {
    fn default() -> Self {
        WindowState::Hidden
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WindowInternalEvent<Message>
where
    Message: std::fmt::Debug + Copy + Clone + Eq + PartialEq,
{
    ShowWindow,
    HideWindow,
    Interior(Message),
}

impl<Message> Widget<WindowInternalEvent<Message>> for Element<Message>
where
    Message: std::fmt::Debug + Copy + Clone + Eq + PartialEq,
{
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<WindowInternalEvent<Message>>> {
        self.handle_event(internal_state, input, event)
            .map(|opt: Option<Message>| opt.map(WindowInternalEvent::Interior))
    }

    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()> {
        Widget::<Message>::draw_frame(self, internal_state, frame)
    }

    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        Widget::<Message>::dimensions(self, internal_state, max_size)
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        Widget::<Message>::set_top_left_position(
            self,
            internal_state,
            position,
        );
    }
}

impl<Message> Into<Element<WindowInternalEvent<Message>>> for Element<Message>
where
    Message: 'static + std::fmt::Debug + Copy + Clone + Eq + PartialEq,
{
    fn into(self) -> Element<WindowInternalEvent<Message>> {
        Element::new(self)
    }
}

/// A window is a collapsable panel.
pub struct Window<Message>
where
    Message: 'static + std::fmt::Debug + Copy + Clone + Eq + PartialEq,
{
    id: Id,
    font: Font,
    title: String,
    contents: Option<Element<Message>>,
    current_view: Option<Element<WindowInternalEvent<Message>>>,
    background_color: Vec4,
}

impl<Message> Window<Message>
where
    Message: 'static + std::fmt::Debug + Copy + Clone + Eq + PartialEq,
{
    pub fn new<T>(
        font: Font,
        title: T,
        view: impl Into<Element<Message>>,
    ) -> Self
    where
        T: Into<String>,
    {
        let owned_title = title.into();
        let id = gen_id!(&owned_title);
        Self {
            id,
            font,
            title: owned_title,
            contents: Some(view.into()),
            current_view: None,
            background_color: vec4(0.0, 0.0, 0.0, 0.3),
        }
    }

    builder_field!(id, Id);
    builder_field!(background_color, Vec4);

    /// Generate a button with a text label.
    fn text_button<T>(
        &self,
        id: Id,
        on_click: WindowInternalEvent<Message>,
        text: T,
    ) -> Button<WindowInternalEvent<Message>>
    where
        T: AsRef<str>,
    {
        let label = Label::new(&self.font, &text)
            .with_padding(self.font.line_height() * 0.125);
        Button::new(id, label).on_click(on_click)
    }

    fn view(
        &mut self,
        internal_state: &mut InternalState,
    ) -> Element<WindowInternalEvent<Message>> {
        // Use the same id for the show and hide buttons. This preserves state
        // even when the button text changes and it means that when the button
        // changes from 'show' to 'hide' the user doesn't need to move the
        // mouse to re-trigger the hover state on the new button.
        let toggle_id = gen_id!(&format!("{} button", self.title));

        let state = internal_state.get_state::<WindowState>(&self.id);
        let contents: Element<WindowInternalEvent<Message>> = match state {
            WindowState::Hidden => {
                // render just the top bar
                Col::new()
                    .child(self.text_button(
                        toggle_id,
                        WindowInternalEvent::ShowWindow,
                        format!("{} [show]", self.title),
                    ))
                    .into()
            }
            WindowState::Visible => {
                // render the visible part of the window
                Col::new()
                    .child(self.text_button(
                        toggle_id,
                        WindowInternalEvent::HideWindow,
                        format!("{} [hide]", self.title),
                    ))
                    .child(
                        self.contents
                            .take()
                            .unwrap()
                            .with_padding(self.font.line_height() * 0.5),
                    )
                    .into()
            }
        };

        Panel::new(contents).color(self.background_color).into()
    }
}

impl<Message> Widget<Message> for Window<Message>
where
    Message: 'static + std::fmt::Debug + Copy + Clone + Eq + PartialEq,
{
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<Message>> {
        if self.current_view.is_none() {
            self.current_view = Some(self.view(internal_state));
        }

        let internal_message: Option<WindowInternalEvent<Message>> = self
            .current_view
            .as_mut()
            .unwrap()
            .handle_event(internal_state, input, event)?;

        let message = match internal_message {
            Some(WindowInternalEvent::ShowWindow) => {
                *internal_state.get_state_mut::<WindowState>(&self.id) =
                    WindowState::Visible;
                None
            }
            Some(WindowInternalEvent::HideWindow) => {
                *internal_state.get_state_mut::<WindowState>(&self.id) =
                    WindowState::Hidden;
                None
            }
            Some(WindowInternalEvent::Interior(msg)) => Some(msg),
            None => None,
        };

        Ok(message)
    }

    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()> {
        Widget::<WindowInternalEvent<Message>>::draw_frame(
            self.current_view.as_ref().unwrap(),
            internal_state,
            frame,
        )?;
        Ok(())
    }

    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions {
        if self.current_view.is_none() {
            self.current_view = Some(self.view(internal_state));
        }

        Widget::<WindowInternalEvent<Message>>::dimensions(
            self.current_view.as_mut().unwrap(),
            internal_state,
            max_size,
        )
    }

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    ) {
        if self.current_view.is_none() {
            self.current_view = Some(self.view(internal_state));
        }

        Widget::<WindowInternalEvent<Message>>::set_top_left_position(
            self.current_view.as_mut().unwrap(),
            internal_state,
            position,
        );
    }
}

impl<Message> Into<Element<Message>> for Window<Message>
where
    Message: std::fmt::Debug + Copy + Clone + Eq + PartialEq,
{
    fn into(self) -> Element<Message> {
        Element::new(self)
    }
}
