use serde::{Deserialize, Serialize};

use crate::thumb_gen::thumbnail::Thumbnail;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    pub id: String, // nanoid
    pub name: String,
    pub size: u64,

    pub thumbs: Vec<Thumbnail>,
}
