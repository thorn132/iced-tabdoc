use iced::{widget, Alignment, Element, Length};

use crate::config::Config;

mod create;
mod home;

pub enum Tab {
    Home,
    Create(create::State),
}

#[derive(Debug, Clone)]
pub enum Message {
    Home(home::Message),
    Create(create::Message),
}

pub fn update(tab: &mut Tab, config: &mut Config, message: Message) {
    match (tab, message) {
        (Tab::Home, Message::Home(msg)) => home::update(config, msg),
        (Tab::Create(tab), Message::Create(msg)) => tab.update(msg),
        _ => unreachable!(),
    }
}

pub fn view<'a>(tab: &'a Tab, config: &'a Config) -> Element<'a, Message> {
    let content = match tab {
        Tab::Home => home::view(config).map(Message::Home),
        Tab::Create(tab) => tab.view().map(Message::Create),
    };

    widget::container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center)
        .into()
}

pub fn new_home_tab() -> Tab {
    Tab::Home
}

pub fn new_create_tab() -> Tab {
    Tab::Create(create::State::new())
}
