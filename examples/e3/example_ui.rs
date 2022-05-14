use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        gen_id,
        ui::{
            widgets::{
                Align, Button, Element, Label, Panel, Row, VAlignment,
                WithPadding,
            },
            Font, UIState,
        },
        vec4,
    },
};

pub struct ExampleUi {
    em: f32,
    font: Font,
    is_fullscreen: bool,
    count: i32,
}

impl ExampleUi {
    pub fn new(
        content_scale: f32,
        asset_loader: &mut AssetLoader,
    ) -> Result<Self> {
        let em = 16.0 * content_scale;
        let font = Font::from_font_file(
            "assets/Roboto-Regular.ttf",
            1.0 * em,
            asset_loader,
        )?;
        Ok(Self {
            em,
            font,
            is_fullscreen: false,
            count: 0,
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ExampleMessage {
    ToggleFullscreen,
    Increment,
    Decrement,
}

impl UIState for ExampleUi {
    type Message = ExampleMessage;

    fn view(&self) -> Element<Self::Message> {
        let message = if self.is_fullscreen { "-" } else { "+" };
        let label =
            Label::new(&self.font, message).with_padding(0.25 * self.em);

        let toggle_fullscreen_button = Button::new(gen_id!(), label)
            .color(vec4(0.1, 0.1, 0.1, 1.0))
            .hover_color(vec4(0.3, 0.3, 0.3, 1.0))
            .pressed_color(vec4(0.5, 0.5, 0.5, 1.0))
            .on_click(ExampleMessage::ToggleFullscreen)
            .with_padding(1.0 * self.em);

        let plus_one_button = Button::new(
            gen_id!(),
            Label::new(&self.font, "+1").with_padding(0.25 * self.em),
        )
        .color(vec4(0.1, 0.1, 0.1, 1.0))
        .hover_color(vec4(0.3, 0.3, 0.3, 1.0))
        .pressed_color(vec4(0.5, 0.5, 0.5, 1.0))
        .on_click(ExampleMessage::Increment)
        .with_padding(1.0 * self.em);

        let minus_one_button = Button::new(
            gen_id!(),
            Label::new(&self.font, "-1").with_padding(0.25 * self.em),
        )
        .color(vec4(0.1, 0.1, 0.1, 1.0))
        .hover_color(vec4(0.3, 0.3, 0.3, 1.0))
        .pressed_color(vec4(0.5, 0.5, 0.5, 1.0))
        .on_click(ExampleMessage::Decrement)
        .with_padding(1.0 * self.em);

        let row = Row::new()
            .child(plus_one_button)
            .child(
                Label::new(&self.font, format!("{}", self.count))
                    .with_padding(self.em),
            )
            .child(minus_one_button)
            .child(toggle_fullscreen_button);

        Align::new(Panel::new(row))
            .vertical_alignment(ccthw::ui::widgets::VAlignment::Center)
            .horizontal_alignment(ccthw::ui::widgets::HAlignment::Center)
            .into()
    }

    fn update(&mut self, message: &ExampleMessage) {
        match *message {
            ExampleMessage::ToggleFullscreen => {
                self.is_fullscreen = !self.is_fullscreen;
            }
            ExampleMessage::Increment => {
                self.count += 1;
            }
            ExampleMessage::Decrement => {
                self.count -= 1;
            }
        }
    }
}
