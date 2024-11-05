use std::{collections::BTreeMap, mem, path::PathBuf};

use iced::{
    widget::{
        self, button, column, row,
        scrollable::{Direction, Scrollbar},
    },
    Alignment, Element, Length, Task,
};

use crate::{
    config::Config,
    documents::{self, Documents},
};

mod create;
mod home;

pub enum Tab {
    Home,
    Create(create::State),
    Document(PathBuf),
}

impl Tab {
    fn label<'a>(&'a self) -> &'a str {
        match self {
            Tab::Home => "Home",
            Tab::Create(_) => "New",
            Tab::Document(path) => path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or_default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Home(usize, home::Message),
    Create(usize, create::Message),
    Document(usize, documents::Message),
    SwitchTo(usize),
    Close(usize),
}

pub struct Tabs {
    tabs: BTreeMap<usize, Tab>,
    next_id: usize,
    history: Vec<usize>,
    current: Option<usize>,
}

impl Tabs {
    pub fn new(tabs: Vec<Tab>) -> Self {
        let mut result = Self {
            tabs: BTreeMap::new(),
            next_id: 0,
            history: Vec::new(),
            current: None,
        };

        for tab in tabs {
            result.push(tab);
        }

        result
    }

    pub fn switch_to(&mut self, key: usize) {
        self.current.replace(key);
        self.history.push(key);
    }

    pub fn switch_to_first(&mut self, predicate: impl Fn(&Tab) -> bool) -> bool {
        if let Some(&key) = self
            .tabs
            .iter()
            .find(|(_, tab)| predicate(tab))
            .map(|(key, _)| key)
        {
            self.switch_to(key);
            true
        } else {
            false
        }
    }

    pub fn is_current(&self, key: usize) -> bool {
        self.current.is_some_and(|k| k == key)
    }

    pub fn current(&self) -> Option<(usize, &Tab)> {
        self.current
            .and_then(|key| self.tabs.get(&key).map(|tab| (key, tab)))
    }

    pub fn push(&mut self, tab: Tab) {
        self.tabs.insert(self.next_id, tab);
        self.switch_to(self.next_id);
        self.next_id += 1;
    }

    pub fn close(&mut self, key: usize) -> Option<Tab> {
        if self.tabs.contains_key(&key) {
            self.history.retain(|&k| k != key);
            self.history.dedup();
            if self.is_current(key) {
                if let Some(next_key) = self.history.pop() {
                    self.switch_to(next_key);
                } else {
                    self.current.take();
                }
            }
            self.tabs.remove(&key)
        } else {
            None
        }
    }

    pub fn clear(&mut self) -> Vec<Tab> {
        self.switch_to(0);
        let old_tabs = mem::replace(&mut self.tabs, BTreeMap::new());
        old_tabs.into_values().collect()
    }

    pub fn update(
        &mut self,
        config: &mut Config,
        documents: &mut Documents,
        message: Message,
    ) -> Task<Message> {
        match message {
            Message::Home(key, msg) => {
                if let Some(Tab::Home) = self.tabs.get(&key) {
                    home::update(config, msg);
                }
            }
            Message::Create(key, msg) => {
                if let Some(Tab::Create(tab)) = self.tabs.get_mut(&key) {
                    return tab.update(msg).map(move |msg| Message::Create(key, msg));
                }
            }
            Message::Document(key, msg) => {
                if let Some(Tab::Document(path)) = self.tabs.get_mut(&key) {
                    if let Some(doc) = documents.get_mut(path) {
                        doc.update(msg);
                    }
                }
            }
            Message::SwitchTo(key) => self.switch_to(key),
            Message::Close(key) => {
                self.close(key);
            }
        }
        Task::none()
    }

    fn tab_bar(&self) -> Element<'_, Message> {
        widget::scrollable(row(self.tabs.iter().map(|(&key, tab)| {
            button(
                row![
                    tab.label(),
                    button("X")
                        .style(button::text)
                        .on_press_with(move || Message::Close(key))
                ]
                .align_y(Alignment::Center)
                .spacing(4),
            )
            .style(if self.is_current(key) {
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

    pub fn view<'a>(
        &'a self,
        config: &'a Config,
        documents: &'a Documents,
    ) -> Element<'a, Message> {
        let tab_bar = self.tab_bar();

        let current_tab = if let Some((key, tab)) = self.current() {
            match tab {
                Tab::Home => home::view(config).map(move |msg| Message::Home(key, msg)),
                Tab::Create(tab) => tab.view().map(move |msg| Message::Create(key, msg)),
                Tab::Document(path) => documents
                    .get(path)
                    .unwrap()
                    .view()
                    .map(move |msg| Message::Document(key, msg)),
            }
        } else {
            "No Open Tabs".into()
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

pub fn new_document_tab(path: PathBuf) -> Tab {
    Tab::Document(path)
}
