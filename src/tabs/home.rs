use iced::{
    border::rounded,
    widget::{self, column, container, row, text, toggler},
    Alignment, Element, Theme,
};

use crate::config::Config;

#[derive(Debug, Clone)]
pub enum Message {
    ChangeShowOnStartup(bool),
}

pub fn update(config: &mut Config, message: Message) {
    match message {
        Message::ChangeShowOnStartup(b) => config.show_home_on_startup = b,
    }
}

pub fn view(config: &Config) -> Element<'_, Message> {
    widget::container(
        column![
            text("Welcome to TabDoc").size(40),
            row![
                "Show Home on Startup",
                toggler(config.show_home_on_startup).on_toggle(Message::ChangeShowOnStartup)
            ]
            .align_y(Alignment::Center)
            .spacing(10),
        ]
        .padding(50),
    )
    .style(|t: &Theme| container::dark(t).border(rounded(20)))
    .into()
}
