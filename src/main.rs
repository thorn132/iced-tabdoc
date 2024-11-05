use std::path::PathBuf;

use config::Config;
use documents::{Document, Documents};
use file_utils::browse_file;
use iced::{
    advanced::graphics::futures::event, application, widget::column, window, Element, Event,
    Subscription, Task,
};

mod config;
mod documents;
mod file_utils;
mod tabs;
mod toolbar;

fn main() -> iced::Result {
    application("TabDoc", App::update, App::view)
        .exit_on_close_request(false)
        .subscription(App::subscription)
        .run_with(App::new)
}

struct App {
    config: Config,
    tabs: tabs::Tabs,
    documents: Documents,
}

#[derive(Debug, Clone)]
enum Message {
    Tab(tabs::Message),
    Toolbar(toolbar::Message),
    OpenFile(PathBuf),
    FileOpened(PathBuf, Document),
    FileError(PathBuf),
    Quit,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let config = Config::load();
        let tabs = if config.show_home_on_startup {
            vec![tabs::new_home_tab()]
        } else {
            vec![]
        };
        let tabs = tabs::Tabs::new(tabs);
        let documents = Documents::new();

        (
            App {
                config,
                tabs,
                documents,
            },
            Task::none(),
        )
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
            Message::Tab(msg) => {
                return self
                    .tabs
                    .update(&mut self.config, &mut self.documents, msg)
                    .map(Message::Tab)
            }
            Message::Toolbar(msg) => match toolbar::update(&mut self.tabs, msg) {
                toolbar::Action::OpenFile => {
                    return Task::perform(browse_file(), Message::OpenFile)
                }
                _ => {}
            },
            Message::OpenFile(path) => {
                self.tabs.push(tabs::new_document_tab(path.clone()));
                if !self.documents.is_open(&path) {
                    return Task::perform(self.documents.open(path.clone()), move |doc| {
                        if let Some(doc) = doc {
                            Message::FileOpened(path.clone(), doc)
                        } else {
                            Message::FileError(path.clone())
                        }
                    });
                }
            }
            Message::FileOpened(path, doc) => {
                self.documents.replace(path, doc);
            }
            Message::FileError(path) => {
                println!("Failed to open {path:?}");
            }
            Message::Quit => {
                self.config.save();
                return window::get_latest().and_then(window::close);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            toolbar::view().map(Message::Toolbar),
            self.tabs
                .view(&self.config, &self.documents)
                .map(Message::Tab)
        ]
        .into()
    }
}
