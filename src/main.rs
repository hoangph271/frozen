use iced::{executor, Application, Command, Element};

mod lib;
use lib::dom_parser::parse_dom;

#[derive(Default)]
struct Counter {}

const HTML: &str = "
    <h4>Frozen</h4>
    <quote>The content focused browser...!</quote>
    <div>
        There isn't any meaningful functionality for now...! :\"<
    </div>
";

impl Application for Counter {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();

    fn title(&self) -> String {
        String::from("#Frozen")
    }

    fn update(&mut self, _message: ()) -> Command<()> {
        Command::none()
    }

    fn view(&mut self) -> Element<()> {
        parse_dom(HTML).expect("parse_dom failed")
    }

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Counter {}, Command::none())
    }
}

fn main() -> Result<(), iced::Error> {
    let mut settings = iced::Settings::default();

    settings.window.resizable = true;
    settings.window.size = (600, 400);

    Counter::run(settings)
}
