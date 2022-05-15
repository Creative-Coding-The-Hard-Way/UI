pub use crate::{
    gen_id,
    ui::{
        id::id_hash,
        widgets::{
            Align, Button, Col, Element, HAlignment, HSplit, Label,
            PaddedWidget, Panel, Row, VAlignment, Widget, Window, WithPadding,
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
pub fn text_button<Message, T>(font: &Font, text: &T) -> Button<Message>
where
    Message: 'static,
    T: AsRef<str>,
{
    let id = gen_id!(text.as_ref());
    Button::new(
        id,
        label(font, text).with_padding(font.line_height() * 0.25),
    )
}

/// Create a text label.
pub fn label<T>(font: &Font, text: &T) -> Label
where
    T: AsRef<str>,
{
    Label::new(font, text)
}

/// Give a widget a background panel.
pub fn panel<Message>(widget: impl Into<Element<Message>>) -> Panel<Message> {
    Panel::new(widget)
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
