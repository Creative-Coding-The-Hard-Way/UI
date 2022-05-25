pub use crate::{
    gen_id,
    ui::{
        id::id_hash,
        primitives::{Axis, Justify, SpaceBetween},
        widgets::{
            Align, Button, Col, Constraint, Container, Element, HAlignment,
            HSplit, Label, Row, Slider, VAlignment, Widget, Window,
            WithContainer,
        },
        Font, Id,
    },
};

/// Wrap a widget with an align widget.
pub fn align<Message, W>(widget: W) -> Align<Message, W>
where
    W: Widget<Message>,
{
    Align::new(widget)
}

/// Wrap the given widget into an interactive button.
pub fn button<Message, E>(id: Id, contents: E) -> Button<Message>
where
    E: Into<Element<Message>>,
{
    Button::new(id, contents)
}

/// Create a text-button.
pub fn text_button<Message>(
    font: &Font,
    text: impl AsRef<str>,
) -> Button<Message>
where
    Message: 'static,
{
    let id = gen_id!(text.as_ref());
    Button::new(
        id,
        label(font, text)
            .container()
            .padding(font.line_height() * 0.25),
    )
}

/// Create a text label.
pub fn label<T>(font: &Font, text: T) -> Label
where
    T: AsRef<str>,
{
    Label::new(font, text)
}

/// Create a column of widgets.
pub fn col<Message>() -> Col<Message> {
    Col::new()
}

/// Create a row of widgets.
pub fn row<Message>() -> Row<Message> {
    Row::new()
}

pub fn hsplit<Message>() -> HSplit<Message> {
    HSplit::new()
}

pub fn slider<Message>(id: Id, min: f32, max: f32) -> Slider<Message> {
    Slider::new(id, min, max)
}
