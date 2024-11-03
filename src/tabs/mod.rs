use iced::{
    widget::{
        self, button, column, row,
        scrollable::{Direction, Scrollbar},
    },
    Alignment, Element, Length,
};

use crate::config::Config;

mod create;
mod home;

pub enum Tab {
    Home,
    Create(create::State),
}

impl Tab {
    fn label<'a>(&'a self) -> &'a str {
        match self {
            Tab::Home => "Home",
            Tab::Create(_) => "New",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Home(usize, home::Message),
    Create(usize, create::Message),
    SwitchTo(usize),
}

pub struct Tabs {
    tabs: Vec<Tab>,
    current: usize,
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            tabs: vec![new_home_tab(), new_create_tab()],
            current: 1,
        }
    }

    pub fn update(&mut self, config: &mut Config, message: Message) {
        match message {
            Message::Home(key, msg) => {
                if let Some(Tab::Home) = self.tabs.get(key) {
                    home::update(config, msg);
                }
            }
            Message::Create(key, msg) => {
                if let Some(Tab::Create(tab)) = self.tabs.get_mut(key) {
                    tab.update(msg);
                }
            }
            Message::SwitchTo(key) => self.current = key,
        }
    }

    fn tab_bar(&self) -> Element<'_, Message> {
        widget::scrollable(row(self.tabs.iter().enumerate().map(|(key, tab)| {
            button(tab.label())
                .style(if key == self.current {
                    button::primary
                } else {
                    button::secondary
                })
                .on_press_with(move || Message::SwitchTo(key))
                .into()
        })))
        .width(Length::Fill)
        .direction(Direction::Horizontal(
            Scrollbar::new().width(2).spacing(0).scroller_width(2),
        ))
        .into()
    }

    pub fn view<'a>(&'a self, config: &'a Config) -> Element<'a, Message> {
        let tab_bar = self.tab_bar();

        let current_tab = match self.tabs.get(self.current).unwrap() {
            Tab::Home => home::view(config).map(|msg| Message::Home(self.current, msg)),
            Tab::Create(tab) => tab.view().map(|msg| Message::Create(self.current, msg)),
        };

        let content = widget::container(current_tab)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center);

        column![tab_bar, content].into()
    }
}

pub fn new_home_tab() -> Tab {
    Tab::Home
}

pub fn new_create_tab() -> Tab {
    Tab::Create(create::State::new())
}
