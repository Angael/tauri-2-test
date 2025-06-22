// FFMPEG commands in progress
// ffmpeg -i in.mp4 -vf "fps=1/duration*$tiles,scale=160:90:force_original_aspect_ratio=decrease,pad=160:90:(ow-iw)/2:(oh-ih)/2:black,tile=${cols}x${rows}" -frames:v 1 preview.webp

// scale can be also like
// scale 360:-2

// best tiles?
// probably 3x3

use ffmpeg_sidecar::command::FfmpegCommand;
use image::{imageops::FilterType, ImageFormat, ImageReader};
use std::error::Error;
use std::path::Path;

pub fn gen_ffmpeg_vid_tiled_thumb(file_absolute_path: String, thumbnail_dir: &Path) {
    println!("do_ffmpeg_stuff: {:?}", file_absolute_path);

    let input = file_absolute_path.clone();
    let output_name = thumbnail_dir.join("thumbnail.webp");

    let mut binding = FfmpegCommand::new();
    let _command = binding
        .hide_banner()
        .overwrite()
        .input(input)
        .args(["-vf", "fps=1,scale=160:-2,tile=3x3"])
        .frames(1)
        .output(output_name.to_str().unwrap());

    // cli commands would look like this:
    // ffmpeg -i in-high.mp4 -vf "fps=1,scale=160:-2,tile=3x3" -frames:v 1 out.webp

    println!("FFmpeg command: {:?}", _command);

    let _ = _command.spawn().unwrap().wait();
}

pub fn gen_image_thumb(
    file_absolute_path: String,
    thumbnail_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    println!("Generating image thumbnail for: {:?}", file_absolute_path);

    let img = ImageReader::open(file_absolute_path)?.decode()?;

    let thumbnail = img.resize_to_fill(256, 256, FilterType::Lanczos3);

    let thumbnail_path = thumbnail_dir.join("thumbnail.avif");

    thumbnail.save_with_format(&thumbnail_path, ImageFormat::Avif)?;

    Ok(())
}
