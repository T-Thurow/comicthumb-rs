mod extractor;
mod utils;

use extractor::get_extractor;
use image::ImageFormat;
use std::{env, process::ExitCode};
use utils::{generate_tumbnail, guess_cover};

const DEFAULT_SIZE: u32 = 128;

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("Usage: <input archive> <output path> [size]");
        return ExitCode::FAILURE;
    }

    let in_path = &args[1];
    let out_path = &args[2];
    let size = if args.len() == 4 {
        args[3].parse().unwrap_or(DEFAULT_SIZE)
    } else {
        DEFAULT_SIZE
    };

    let extractor = get_extractor(in_path).expect("Could not create extractor");
    let files = extractor.get_files().expect("Could not read archive");
    let chosen = guess_cover(&files).expect("Could not guess cover");
    let raw = extractor.extract(&chosen).expect("Failed to extract file");
    let img = image::load_from_memory(&raw).expect("Failed to load image");
    let thumbnail = generate_tumbnail(&img, size);

    thumbnail
        .save_with_format(out_path, ImageFormat::Png)
        .expect("Failed to save image");
    ExitCode::SUCCESS
}
