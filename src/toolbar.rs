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
    OpenFile,
    CloseAll,
}

pub enum Action {
    None,
    OpenFile,
}

pub fn update(tabs: &mut Tabs, message: Message) -> Action {
    match message {
        Message::GoHome => {
            if !tabs.switch_to_first(|tab| matches!(tab, Tab::Home)) {
                tabs.push(tabs::new_home_tab());
            }
        }
        Message::AddCreateTab => tabs.push(tabs::new_create_tab()),
        Message::OpenFile => return Action::OpenFile,
        Message::CloseAll => {
            tabs.clear();
        }
    }
    Action::None
}

pub fn view<'a>() -> Element<'a, Message> {
    row![
        button("Home").on_press(Message::GoHome),
        button("New").on_press(Message::AddCreateTab),
        button("Open").on_press(Message::OpenFile),
        button("Close All").on_press(Message::CloseAll)
    ]
    .spacing(10)
    .padding(bottom(10))
    .into()
}
