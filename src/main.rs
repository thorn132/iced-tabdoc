use config::Config;
use iced::{
    advanced::graphics::futures::event,
    application,
    border::rounded,
    widget::{
        self, column,
        container::{self},
        row, text, toggler,
    },
    window, Alignment, Element, Event, Length, Subscription, Task, Theme,
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
        widget::container(
            widget::container(
                column![
                    text("Welcome to TabDoc").size(40),
                    row![
                        "Show Home on Startup",
                        toggler(self.config.show_home_on_startup)
                            .on_toggle(Message::ChangeShowHomeOnStartup)
                    ]
                    .align_y(Alignment::Center)
                    .spacing(10),
                ]
                .padding(50),
            )
            .style(|t: &Theme| container::dark(t).border(rounded(20))),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
    }
}
