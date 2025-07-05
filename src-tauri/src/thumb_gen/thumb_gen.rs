// FFMPEG commands in progress
// ffmpeg -i in.mp4 -vf "fps=1/duration*$tiles,scale=160:90:force_original_aspect_ratio=decrease,pad=160:90:(ow-iw)/2:(oh-ih)/2:black,tile=${cols}x${rows}" -frames:v 1 preview.webp

// scale can be also like
// scale 360:-2

// best tiles?
// probably 3x3

use ffmpeg_sidecar::command::FfmpegCommand;
use image::{imageops::FilterType, ImageFormat, ImageReader};
use std::path::Path;

use crate::files_in_dirs::file::{File, FileThumbs};

const TILE_SIZE: u32 = 256;
const ROWS: u32 = 4;
const COLS: u32 = 4;

pub fn gen_ffmpeg_vid_tiled_thumb(
    file: &File,
    file_absolute_path: String,
    thumbnail_dir: &Path,
) -> Result<FileThumbs, String> {
    println!("do_ffmpeg_stuff: {:?}", file.name);

    let input = file_absolute_path.clone();
    let output_name = thumbnail_dir.join("thumbnail.avif");

    let mut fps: f64 = 1.0; // frames per second
    if let Some(video_stats) = file.video_stats.as_ref() {
        fps = f64::from(ROWS * COLS) / video_stats.dur;
    }

    let mut binding = FfmpegCommand::new();
    // Without filling bars:
    // let vf_arg = format!(
    //     "fps={},scale={}:{},tile={}x{}",
    //     fps, TILE_SIZE, TILE_SIZE, COLS, ROWS
    // );
    let vf_arg = format!(
        "fps={fps},scale={w}:{h}:force_original_aspect_ratio=decrease,pad={w}:{h}:-1:-1,tile={cols}x{rows}",
        fps = fps,
        w = TILE_SIZE,
        h = TILE_SIZE,
        cols = COLS,
        rows = ROWS
    );
    let _command = binding
        .hide_banner()
        .overwrite()
        .input(input)
        .args(["-vf", &vf_arg, "-crf", "45"])
        .frames(1)
        .output(output_name.to_str().unwrap());

    // cli commands would look like this:
    // ffmpeg -i in-d.mp4 -y -hide_banner -vf "fps=2,scale=256:256,tile=4x4" -frames:v 1 -crf 50 out.avif
    // ffmpeg -i in-d.mp4 -y -hide_banner -vf "fps=3,scale=256:256,setpts=PTS/6" -an -crf 50 out.webm

    println!("FFmpeg command: {:?}", _command);

    let _ = _command.spawn().unwrap().wait();

    Ok(FileThumbs {
        t256: true,
        s256: false,
        s512: false,
        t512: false,
        a256: false,
        a512: false,
    })
}

pub fn gen_image_thumb(
    file_absolute_path: String,
    thumbnail_dir: &Path,
) -> Result<FileThumbs, String> {
    println!("Generating image thumbnail for: {:?}", file_absolute_path);

    let img = ImageReader::open(file_absolute_path)
        .map_err(|_err| "Failed to open image")?
        .decode()
        .map_err(|_err| "Failed to decode image")?;
    // let width = img.width();
    // let height = img.height();

    let thumbnail = img.resize_to_fill(TILE_SIZE, TILE_SIZE, FilterType::Lanczos3);

    let thumbnail_path = thumbnail_dir.join("thumbnail.avif");

    thumbnail
        .save_with_format(&thumbnail_path, ImageFormat::Avif)
        .map_err(|err| format!("Failed to save thumbnail: {}", err))?;

    Ok(FileThumbs {
        t256: false,
        s256: true,
        s512: false,
        t512: false,
        a256: false,
        a512: false,
    })
}
