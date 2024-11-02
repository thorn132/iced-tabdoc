use iced::Element;

pub struct State {}

#[derive(Debug, Clone)]
pub enum Message {}

impl State {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        "".into()
    }
}
