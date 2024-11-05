use std::path::PathBuf;

pub async fn browse_file() -> PathBuf {
    rfd::AsyncFileDialog::new()
        .set_title("Choose a file...")
        .add_filter("Image", &["bmp", "jpg"])
        .add_filter("Text", &["txt"])
        .add_filter("All Files", &["*"])
        .pick_file()
        .await
        .map(|x| PathBuf::from(x))
        .unwrap_or(PathBuf::new())
}

pub async fn browse_directory() -> PathBuf {
    rfd::AsyncFileDialog::new()
        .set_title("Choose a directory...")
        .pick_folder()
        .await
        .map(|x| PathBuf::from(x))
        .unwrap_or_default()
}
