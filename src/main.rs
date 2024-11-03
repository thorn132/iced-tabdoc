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
    tabs: tabs::Tabs,
}

#[derive(Debug, Clone)]
enum Message {
    Tab(tabs::Message),
    Quit,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let config = Config::load();
        let tabs = tabs::Tabs::new();

        (App { config, tabs }, Task::none())
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
            Message::Tab(msg) => self.tabs.update(&mut self.config, msg),
            Message::Quit => {
                self.config.save();
                return window::get_latest().and_then(window::close);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        self.tabs.view(&self.config).map(Message::Tab)
    }
}
