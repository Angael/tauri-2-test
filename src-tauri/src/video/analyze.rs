use std::path::PathBuf;

use ffprobe::ffprobe;
use serde::{Deserialize, Serialize};

fn approx_video_bitrate(file_size_bytes: u64, duration_secs: f64, audio_fraction: f64) -> u32 {
    let bits = (file_size_bytes as f64) * 8.0 * (1.0 - audio_fraction);
    (bits / duration_secs).round() as u32
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VideoStats {
    /** Duration in seconds */
    pub dur: f64,

    /** Resolution of the video (width, height) in px */
    pub res: (u16, u16),

    /** Bitrate in kbps */
    pub br: u32,
}

pub fn analyze_video(path: PathBuf) -> Result<VideoStats, String> {
    let filename = (&path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let info = match ffprobe(&path) {
        Ok(info) => info,
        Err(e) => {
            eprintln!("Failed to analyze video '{}': {}", filename, e);
            return Err("Video analysis failed".to_string());
        }
    };

    let v_stream = match info
        .streams
        .iter()
        .find(|s| s.codec_type == Some("video".to_string()))
    {
        Some(stream) => stream,
        None => {
            eprintln!("No video stream found in '{}'", filename);
            return Err("No video stream found".to_string());
        }
    };

    let video_stats = VideoStats {
        dur: info
            .format
            .duration
            .as_ref()
            .and_then(|d| d.parse::<f64>().ok())
            .unwrap_or(0.0),
        res: (
            v_stream.width.unwrap_or(0) as u16,
            v_stream.height.unwrap_or(0) as u16,
        ),
        br: v_stream
            .bit_rate
            .as_ref()
            .and_then(|bit_rate| bit_rate.parse::<u32>().ok())
            .unwrap_or_else(|| {
                if let Some(duration_str) = &info.format.duration {
                    if let (Ok(size), Ok(duration)) = (
                        info.format
                            .size
                            .expect("FFprobe didn't supply size")
                            .parse::<u64>(),
                        duration_str.parse::<f64>(),
                    ) {
                        return approx_video_bitrate(size, duration, 0.08_f64);
                    }
                }
                0
            }),
    };

    Ok(video_stats)
}
