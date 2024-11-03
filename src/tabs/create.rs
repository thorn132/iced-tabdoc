use std::path::PathBuf;

use iced::{
    border::rounded,
    widget::{self, button, column, container, pick_list, row, text, text_input},
    Alignment, Element, Length, Theme,
};

pub struct State {
    file_name: String,
    file_type: Option<&'static str>,
    directory: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeFileName(String),
    ChangeFileType(&'static str),
    ChangeDirectory(PathBuf),
}

impl State {
    pub fn new() -> Self {
        Self {
            file_name: String::new(),
            file_type: None,
            directory: None,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChangeFileName(n) => self.file_name = n,
            Message::ChangeFileType(t) => self.file_type = Some(t),
            Message::ChangeDirectory(d) => self.directory = Some(d),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        widget::container(
            column![
                text("Create File").size(40),
                row![
                    text_input("File Name", &self.file_name).on_input(Message::ChangeFileName),
                    pick_list(["Text", "Bitmap"], self.file_type, Message::ChangeFileType)
                ]
                .spacing(10),
                row![
                    text_input(
                        "Directory",
                        self.directory
                            .as_ref()
                            .and_then(|d| d.to_str())
                            .unwrap_or_default()
                    ),
                    button("Browse")
                ]
                .spacing(10),
                button(
                    text("Create File")
                        .width(Length::Fill)
                        .align_x(Alignment::Center)
                )
            ]
            .width(Length::FillPortion(3))
            .padding(50)
            .spacing(10),
        )
        .width(500)
        .style(|t: &Theme| container::dark(t).border(rounded(20)))
        .into()
    }
}
