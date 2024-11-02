use config::Config;
use iced::{
    advanced::graphics::futures::event, application, window, Element, Event, Subscription, Task,
};

mod config;
mod tabs;

fn main() -> iced::Result {
    application("TabDoc", App::update, App::view)
        .exit_on_close_request(false)
        .subscription(App::subscription)
        .run_with(App::new)
}

struct App {
    config: Config,
}

#[derive(Debug, Clone)]
enum Message {
    Home(tabs::home::Message),
    Quit,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let config = Config::load();
        (App { config }, Task::none())
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|e, _, _| {
            if let Event::Window(window::Event::CloseRequested) = e {
                Some(Message::Quit)
            } else {
                None
            }
        })
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Home(msg) => tabs::home::update(&mut self.config, msg),
            Message::Quit => {
                self.config.save();
                return window::get_latest().and_then(window::close);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        tabs::home::view(&self.config).map(Message::Home)
    }
}
