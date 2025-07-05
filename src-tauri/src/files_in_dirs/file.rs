use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FileThumbs {
    // Static thumbnails
    pub s256: bool,
    pub s512: bool,

    // Tiled thumbnails
    pub t256: bool,
    pub t512: bool,

    // Animated thumbnails
    pub a256: bool,
    pub a512: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    pub id: String, // nanoid
    pub name: String,
    pub size: u64,

    // #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbs: Option<FileThumbs>,
    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // TODO: This won't work correctly, as there is no full path to the original file without the directory
    // pub original_file: Option<Box<File>>, // Placeholder for video analysis data
    // #[serde(default, skip_serializing_if = "Option::is_none")]
}
