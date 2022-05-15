use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        ui::{widgets::prelude::*, UIState},
    },
};

pub struct ExampleUi {
    em: f32,
    font: Font,
    is_fullscreen: bool,
    pub count: i32,
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

    fn text_button(
        &self,
        text: &impl AsRef<str>,
        on_click: ExampleMessage,
    ) -> Element<ExampleMessage> {
        text_button(&self.font, text)
            .on_click(on_click)
            .with_padding(0.25 * self.em)
            .into()
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
        let message = if self.is_fullscreen {
            "Windowed"
        } else {
            "Fullscreen"
        };

        let controls =
            self.text_button(&message, ExampleMessage::ToggleFullscreen);

        let counter = row()
            .child(self.text_button(&"-1", ExampleMessage::Decrement))
            .child(label(&self.font, &format!("{}", self.count)))
            .child(self.text_button(&"+1", ExampleMessage::Increment));

        hsplit()
            .left(align(counter).alignment(HAlignment::Left, VAlignment::Top))
            .right(
                align(controls).alignment(HAlignment::Right, VAlignment::Top),
            )
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
