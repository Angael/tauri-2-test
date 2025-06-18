use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VideoStats {
    /** Duration in seconds */
    /** Resolution of the video (width, height) */
    pub dur: f64,

    /** Resolution of the video (width, height) in px */
    pub res: (u16, u16),

    /** Bitrate in kbps */
    pub br: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct File {
    pub name: String,
    pub size: u64,

    // #[serde(default, skip_serializing_if = "Option::is_none")]
    // TODO: This won't work correctly, as there is no full path to the original file without the directory
    // pub original_file: Option<Box<File>>, // Placeholder for video analysis data
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_stats: Option<VideoStats>, // Placeholder for video analysis data
}

impl File {
    pub fn new(name: String, size: u64) -> Self {
        File {
            name,
            size,
            // original_file: None,
            video_stats: None,
        }
    }
}
