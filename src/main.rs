use iced::{application, Element, Task};

fn main() -> iced::Result {
    application("TabDoc", App::update, App::view).run_with(App::new)
}

struct App;

#[derive(Debug, Clone)]
enum Message {}

impl App {
    fn new() -> (Self, Task<Message>) {
        (App, Task::none())
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        "".into()
    }
}
