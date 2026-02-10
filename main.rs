use anyhow::{Context, Result};
use clap::Parser;
use image::GenericImageView;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input image path
    #[arg(required = true)]
    input: PathBuf,

    /// Number of slices
    #[arg(short, long)]
    n: u32,

    /// Direction to slice: 'v' for vertical, 'h' for horizontal
    #[arg(short, long, default_value = "v")]
    direction: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let img = image::open(&args.input).with_context(|| format!("Failed to open input image {:?}", args.input))?;
    let (width, height) = img.dimensions();

    let prefix = args.input.file_stem().with_context(|| "Invalid input filename")?.to_str().with_context(|| "Invalid UTF-8 in filename")?;
    let ext = args.input.extension().unwrap_or_default().to_str().unwrap_or("");
    let parent = args.input.parent().unwrap_or_else(|| std::path::Path::new("."));

    let is_vertical = args.direction.starts_with('v');

    let total_size = if is_vertical { width } else { height };
    let slice_size = total_size / args.n;
    
    let mut current_pos = 0;

    for i in 0..args.n {
        let actual_slice_size = if i == args.n - 1 {
            total_size - current_pos
        } else {
            slice_size
        };

        // use crop_imm for simpler handling, returns DynamicImage
        let x = if is_vertical { current_pos } else { 0 };
        let y = if is_vertical { 0 } else { current_pos };
        let w = if is_vertical { actual_slice_size } else { width };
        let h = if is_vertical { height } else { actual_slice_size };
        
        let mut sub_img = img.crop_imm(x, y, w, h);

        let output_filename = if ext.is_empty() {
             format!("{}-{}", prefix, i + 1)
        } else {
             format!("{}-{}.{}", prefix, i + 1, ext)
        };
        let output_path = parent.join(&output_filename);
        
        // Handle JPEG limitation: cannot save explicit alpha channel
        if (ext.eq_ignore_ascii_case("jpg") || ext.eq_ignore_ascii_case("jpeg")) && sub_img.color().has_alpha() {
             sub_img = image::DynamicImage::ImageRgb8(sub_img.to_rgb8());
        }

        sub_img.save(&output_path).with_context(|| format!("Failed to save slice to {:?}", output_path))?;

        current_pos += actual_slice_size;
    }

    Ok(())
}

