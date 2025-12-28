use std::path::PathBuf;

#[derive(Debug)]
pub struct Theme {
    pub name: String,
    pub path: PathBuf,
    pub style: String,
    pub image_path: Option<PathBuf>,
    pub data: ThemeData,
    pub current: bool,
}

#[derive(Debug)]
pub struct ThemeData {
    pub name: String,
    pub description: String,
    pub experimental: bool,
}
