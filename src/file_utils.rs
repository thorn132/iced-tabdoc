use std::path::PathBuf;

pub async fn browse_directory() -> PathBuf {
    rfd::AsyncFileDialog::new()
        .set_title("Choose a directory...")
        .pick_folder()
        .await
        .map(|x| PathBuf::from(x))
        .unwrap_or_default()
}
