use iced::{
    padding::bottom,
    widget::{button, row},
    Element,
};

use crate::tabs::{self, Tab, Tabs};

#[derive(Debug, Clone)]
pub enum Message {
    GoHome,
    AddCreateTab,
    CloseAll,
}

pub fn update(tabs: &mut Tabs, message: Message) {
    match message {
        Message::GoHome => {
            if !tabs.switch_to_first(|tab| matches!(tab, Tab::Home)) {
                tabs.push(tabs::new_home_tab());
            }
        }
        Message::AddCreateTab => tabs.push(tabs::new_create_tab()),
        Message::CloseAll => {
            tabs.clear();
        }
    }
}

pub fn view<'a>() -> Element<'a, Message> {
    row![
        button("Home").on_press(Message::GoHome),
        button("New").on_press(Message::AddCreateTab),
        button("Close All").on_press(Message::CloseAll)
    ]
    .spacing(10)
    .padding(bottom(10))
    .into()
}