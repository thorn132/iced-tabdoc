use config::Config;
use iced::{
    advanced::graphics::futures::event, application, widget::toggler, window, Element, Event,
    Subscription, Task,
};

mod config;

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
    ChangeShowHomeOnStartup(bool),
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
            Message::ChangeShowHomeOnStartup(b) => self.config.show_home_on_startup = b,
            Message::Quit => {
                self.config.save();
                return window::get_latest().and_then(window::close);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        toggler(self.config.show_home_on_startup)
            .on_toggle(Message::ChangeShowHomeOnStartup)
            .into()
    }
}
