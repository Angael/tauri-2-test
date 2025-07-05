use serde::{Deserialize, Serialize};

use crate::thumb_gen::thumbnail::Thumbnail;

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

    pub thumbs: Vec<Thumbnail>,
}
