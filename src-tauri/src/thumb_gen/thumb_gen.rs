// FFMPEG commands in progress
// ffmpeg -i in.mp4 -vf "fps=1/duration*$tiles,scale=160:90:force_original_aspect_ratio=decrease,pad=160:90:(ow-iw)/2:(oh-ih)/2:black,tile=${cols}x${rows}" -frames:v 1 preview.webp

// scale can be also like
// scale 360:-2

// best tiles?
// probably 3x3

use ffmpeg_sidecar::command::FfmpegCommand;
use image::{imageops::FilterType, ImageFormat, ImageReader};
use std::path::{Path, PathBuf};

use crate::{
    files_in_dirs::file::File,
    thumb_gen::thumbnail::Thumbnail,
    video::analyze::{analyze_video, VideoStats},
};

const TILE_SIZE: u16 = 256;

/*
cli commands would look like this:
Fill bars:
ffmpeg -i in-d.mp4 -y -hide_banner -vf "fps=2,scale=256:256:force_original_aspect_ratio=decrease,pad=256:256:-1:-1,tile=4x4" -frames:v 1 -crf 50 out-bars.avif
Cover:
ffmpeg -i in-d.mp4 -y -hide_banner -vf "fps=2,scale=256:256:force_original_aspect_ratio=increase,crop=256:256,tile=4x4" -frames:v 1 -crf 50 out-cover.avif
Bad scale to x:y:
ffmpeg -i in-d.mp4 -y -hide_banner -vf "fps=2,scale=256:256,tile=4x4" -frames:v 1 -crf 50 out-scaled.avif
Animated thumbnail:
ffmpeg -i in-d.mp4 -y -hide_banner -vf "fps=3,scale=256:256,setpts=PTS/6" -an -crf 50 out.webm
*/
const MIN_TILES: u8 = 9;
const MAX_TILES: u8 = 36;
fn decide_grid_for_video(video_stats: &VideoStats) -> (u8, u8) {
    let tiles_aprox = (video_stats.dur / 5.0)
        .min(MAX_TILES as f64)
        .max(MIN_TILES as f64) as u8;

    match tiles_aprox {
        // n if n < MIN_TILES => (3, 3), // Minimum grid size
        // n if n > MAX_TILES => (6, 6), // Maximum grid size
        n => {
            let cols = (n as f64).sqrt().ceil() as u8;
            let rows = (n as f64 / cols as f64).ceil() as u8;
            (cols, rows)
        }
    }
}

pub fn gen_ffmpeg_vid_tiled_thumb(
    file: &File,
    file_absolute_path: String,
    thumbnail_dir: &Path,
) -> Result<Thumbnail, String> {
    let input = file_absolute_path.clone();
    let output_name = thumbnail_dir.join("thumbnail.avif");

    let video_stats: VideoStats = analyze_video(PathBuf::from(&input))
        .map_err(|err| format!("Failed to analyze video: {}", err))?;

    let (cols, rows) = decide_grid_for_video(&video_stats);
    let fps = f64::from(cols * rows) / video_stats.dur;

    let mut binding = FfmpegCommand::new();

    let vf_arg = format!(
        "fps={fps},scale={w}:{h}:force_original_aspect_ratio=increase,crop={w}:{h},tile={cols}x{rows}",
        fps = fps,
        w = TILE_SIZE,
        h = TILE_SIZE,
        cols = cols,
        rows = rows
    );
    let _command = binding
        .hide_banner()
        .overwrite()
        .input(input)
        .args(["-vf", &vf_arg, "-crf", "40"])
        .frames(1)
        .output(output_name.to_str().unwrap());

    println!("FFmpeg command: {:?}", _command);

    let _ = _command.spawn().unwrap().wait();

    Ok(Thumbnail {
        res: (TILE_SIZE, TILE_SIZE),
        grid: Some((cols, rows)),
    })
}

pub fn gen_image_thumb(
    file_absolute_path: String,
    thumbnail_dir: &Path,
) -> Result<Thumbnail, String> {
    let img = ImageReader::open(file_absolute_path)
        .map_err(|_err| "Failed to open image")?
        .decode()
        .map_err(|_err| "Failed to decode image")?;
    // let width = img.width();
    // let height = img.height();

    let thumbnail = img.resize_to_fill(TILE_SIZE.into(), TILE_SIZE.into(), FilterType::Lanczos3);

    let thumbnail_path = thumbnail_dir.join("thumbnail.avif");

    thumbnail
        .save_with_format(&thumbnail_path, ImageFormat::Avif)
        .map_err(|err| format!("Failed to save thumbnail: {}", err))?;

    Ok(Thumbnail {
        res: (TILE_SIZE, TILE_SIZE),
        grid: None,
    })
}
