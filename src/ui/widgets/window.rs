use ::anyhow::Result;

use crate::{
    gen_id,
    ui::{
        id_hash,
        primitives::Justify,
        widgets::{
            Button, Col, ComposedMessage, Composite, CompositeWidget,
            Container, Element, Label, WithContainer,
        },
        Font, Id,
    },
    vec4,
};

/// This type represents the ['Window']'s current visibity state.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WindowState {
    Hidden,
    Visible,
}

impl Default for WindowState {
    /// Window's default to the Hidden state.
    fn default() -> Self {
        WindowState::Hidden
    }
}

/// An internal event used by the Window to toggle the visibility of the
/// internal contents.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WindowEvent {
    ShowWindow,
    HideWindow,
}

/// A Window is a collapsable panel with a title button which toggles the
/// visibility of the contents.
pub struct Window<Message> {
    id: Id,
    font: Font,
    title: String,
    contents: Option<Element<Message>>,
}

impl<Message> Window<Message>
where
    Message: 'static + std::fmt::Debug + Copy + Clone,
{
    pub fn new(font: Font, title: impl Into<String>) -> Self {
        let owned_title = title.into();
        Self {
            id: gen_id!(&owned_title),
            font,
            title: owned_title,
            contents: None,
        }
    }

    pub fn contents(self, contents: impl Into<Element<Message>>) -> Self {
        Self {
            contents: Some(contents.into()),
            ..self
        }
    }

    /// Generate a button with a text label.
    fn text_button<T>(
        &self,
        id: Id,
        on_click: WindowEvent,
        text: T,
    ) -> Button<ComposedMessage<WindowEvent, Message>>
    where
        T: AsRef<str>,
    {
        let label = Label::new(&self.font, &text)
            .container()
            .padding(self.font.line_height() * 0.125);
        Button::new(id, label)
            .on_click(ComposedMessage::Internal(on_click))
            .color(vec4(0.0, 0.0, 0.0, 0.2))
            .hover_color(vec4(1.0, 1.0, 1.0, 0.2))
            .pressed_color(vec4(1.0, 1.0, 1.0, 0.5))
    }
}

impl<Message> CompositeWidget<WindowEvent, Message> for Window<Message>
where
    Message: 'static + std::fmt::Debug + Copy + Clone,
{
    type State = WindowState;

    fn id(&self) -> &Id {
        &self.id
    }

    fn view(
        &mut self,
        state: &Self::State,
    ) -> Element<ComposedMessage<WindowEvent, Message>> {
        // Use the same id for the show and hide buttons. This preserves state
        // even when the button text changes and it means that when the button
        // changes from 'show' to 'hide' the user doesn't need to move the
        // mouse to re-trigger the hover state on the new button.
        let toggle_id = gen_id!(&format!("{} button", self.title));

        match state {
            WindowState::Hidden => {
                // render just the top bar
                Col::new()
                    .child(
                        self.text_button(
                            toggle_id,
                            WindowEvent::ShowWindow,
                            format!("[show] {}", self.title),
                        ),
                        Justify::Begin,
                    )
                    .into()
            }
            WindowState::Visible => {
                let contents: Element<Message> = self
                    .contents
                    .take()
                    .unwrap()
                    .container()
                    .padding(self.font.line_height() * 0.5)
                    .into();

                // render the visible part of the window
                Col::new()
                    .child(
                        self.text_button(
                            toggle_id,
                            WindowEvent::HideWindow,
                            format!("[hide] {}", self.title),
                        ),
                        Justify::Begin,
                    )
                    .child(contents, Justify::Center)
                    .into()
            }
        }
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: WindowEvent,
    ) -> Result<()> {
        match event {
            WindowEvent::HideWindow => {
                *state = WindowState::Hidden;
            }
            WindowEvent::ShowWindow => {
                *state = WindowState::Visible;
            }
        }
        Ok(())
    }
}

impl<Message> WithContainer<Message, Element<Message>> for Window<Message>
where
    Message: 'static + std::fmt::Debug + Copy + Clone,
{
    fn container(self) -> Container<Message, Element<Message>> {
        let result: Element<Message> = self.into();
        result.container()
    }
}

impl<Message> Into<Element<Message>> for Window<Message>
where
    Message: 'static + std::fmt::Debug + Copy + Clone,
{
    fn into(self) -> Element<Message> {
        Element::new(Composite::new(self))
    }
}

impl<Message> Into<Element<Message>>
    for Composite<WindowEvent, Message, Window<Message>>
where
    Message: 'static + std::fmt::Debug + Copy + Clone,
{
    fn into(self) -> Element<Message> {
        Element::new(self)
    }
}
