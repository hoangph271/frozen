use iced::button::{self, Button};
use iced::{Alignment, Column, Element, Sandbox, Text};

#[derive(Default)]
struct Counter {
    value: i32,
    exit_button: button::State,
    increment_pressed: button::State
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
        String::from("Frozen")
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
            .push(Text::new(format!("The `iced-rs` content focused browser, value: {}", self.value.to_string())).size(50))
            .push(
                Button::new(&mut self.increment_pressed, Text::new("Quit"))
                    .on_press(Message::IncrementPressed),
            )
            .into()
    }
}

fn main() -> Result<(), iced::Error> {
    Counter::run(iced::Settings::default())
}
