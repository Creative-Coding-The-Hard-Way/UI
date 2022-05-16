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

    pub fn text_button(
        &self,
        message: impl AsRef<str>,
        on_click: ExampleMessage,
    ) -> impl Into<Element<ExampleMessage>> {
        text_button(&self.font, &message).on_click(on_click)
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
        let em = self.em;

        let message = if self.is_fullscreen {
            "Windowed"
        } else {
            "Fullscreen"
        };

        let window = Window::new(self.font.clone(), "window controls")
            .contents(col().child(
                self.text_button(message, ExampleMessage::ToggleFullscreen),
            ));

        let state = Window::new(self.font.clone(), "state controls").contents(
            row()
                .child(self.text_button("-1", ExampleMessage::Decrement))
                .child(
                    label(&self.font, &format!("{}", self.count))
                        .with_padding(1.0 * em),
                )
                .child(self.text_button("+1", ExampleMessage::Increment)),
        );

        hsplit()
            .right(
                align(hsplit().left(window).right(state))
                    .alignment(HAlignment::Right, VAlignment::Top),
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
