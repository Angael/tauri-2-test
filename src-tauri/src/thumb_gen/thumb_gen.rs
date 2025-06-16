// FFMPEG commands in progress
// ffmpeg -i in.mp4 -vf "fps=1/duration*$tiles,scale=160:90:force_original_aspect_ratio=decrease,pad=160:90:(ow-iw)/2:(oh-ih)/2:black,tile=${cols}x${rows}" -frames:v 1 preview.webp

// scale can be also like
// scale 360:-2

// best tiles?
// probably 3x3

use ffmpeg_sidecar::command::FfmpegCommand;

pub fn gen_ffmpeg_vid_tiled_thumb(file_absolute_path: String) {
    println!("do_ffmpeg_stuff: {:?}", file_absolute_path);

    let input = file_absolute_path.clone();
    let output_name = input.clone() + ".webp";

    let mut binding = FfmpegCommand::new();
    let _command = binding
        .hide_banner()
        .overwrite()
        .input(input)
        .args(&["-vf", "fps=1,scale=160:-2,tile=3x3"])
        .frames(1)
        .output(output_name);

    println!("FFmpeg command: {:?}", _command);

    let _ = _command.spawn().unwrap().wait();
}
