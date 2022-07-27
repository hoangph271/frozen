use iced::button::{self, Button};
use iced::{Alignment, Column, Element, Sandbox, Text};

mod lib;

#[derive(Default)]
struct Counter {
    value: i32,
    exit_button: button::State,
    increment_pressed: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ExitPressed,
    IncrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Rusted Content")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ExitPressed => {
                self.value += 1;
            }
            Message::IncrementPressed => {
                todo!("// ! TODO: Handle exit app...!")
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Alignment::Center)
            .push(
                Button::new(&mut self.exit_button, Text::new("Increment"))
                    .on_press(Message::ExitPressed),
            )
            .push(
                Text::new(format!(
                    "`RC` - The `iced-rs` content focus browser, value: {}",
                    self.value
                ))
                .size(50),
            )
            .push(
                Button::new(&mut self.increment_pressed, Text::new("Exit app"))
                    .on_press(Message::IncrementPressed),
            )
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    Counter::run(iced::Settings::default())
}
