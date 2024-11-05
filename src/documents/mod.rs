use std::{collections::HashMap, future::Future, path::PathBuf};

use iced::{widget, Element};

mod image;
mod text;

#[derive(Debug, Clone)]
pub enum Document {
    Loading,
    Image(image::State),
    Text(text::State),
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Document {
    pub fn update(&mut self, message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        match self {
            Document::Loading => "Loading...".into(),
            Document::Image(state) => widget::image(&state.handle).into(),
            _ => unimplemented!(),
        }
    }
}

pub struct Documents {
    documents: HashMap<PathBuf, Document>,
}

impl Documents {
    pub fn new() -> Self {
        Self {
            documents: HashMap::new(),
        }
    }

    pub fn open(&mut self, path: PathBuf) -> impl Future<Output = Option<Document>> {
        self.documents.insert(path.clone(), Document::Loading);
        Documents::load(path)
    }

    pub fn is_open(&self, path: &PathBuf) -> bool {
        self.documents.contains_key(path)
    }

    pub fn replace(&mut self, path: PathBuf, doc: Document) {
        if let Some(entry) = self.documents.get_mut(&path) {
            *entry = doc;
        }
    }

    pub fn get(&self, path: &PathBuf) -> Option<&Document> {
        self.documents.get(path)
    }

    pub fn get_mut(&mut self, path: &PathBuf) -> Option<&mut Document> {
        self.documents.get_mut(path)
    }

    async fn load(path: PathBuf) -> Option<Document> {
        match path
            .extension()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
        {
            "bmp" | "jpg" => Some(Document::Image(image::State::load(path)?)),
            _ => None,
        }
    }
}
