use std::path::PathBuf;

use iced::widget::image::Handle;
use image::RgbaImage;

#[derive(Debug, Clone)]
pub struct State {
    pub handle: Handle,
    pub width: u32,
    pub height: u32,
}

impl State {
    pub fn load(path: PathBuf) -> Option<Self> {
        let image: RgbaImage = image::ImageReader::open(path)
            .ok()?
            .decode()
            .ok()?
            .into_rgba8();

        let width = image.width();
        let height = image.height();
        let handle = Handle::from_rgba(width, height, image.into_vec());

        Some(Self {
            handle,
            width,
            height,
        })
    }
}
