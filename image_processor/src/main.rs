use anyhow::{Context, Result};
use clap::Parser;
use image::RgbaImage;
use log::{debug, info};
use std::path::PathBuf;

mod plugin_loader;

#[derive(Parser)]
struct Args {
    /// Path to input PNG image
    #[arg(long)]
    input: PathBuf,

    /// Path to save output PNG image
    #[arg(long)]
    output: PathBuf,

    /// Plugin name (without extension)
    #[arg(long)]
    plugin: String,

    /// Path to JSON parameters file
    #[arg(long)]
    params: PathBuf,

    /// Directory containing plugins
    #[arg(long, default_value = "target/debug")]
    plugin_path: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    // Load PNG image and convert to RGBA8
    info!("Loading image from: {}", args.input.display());
    let img = image::open(&args.input)
        .with_context(|| format!("Failed to load image: {}", args.input.display()))?
        .into_rgba8();

    // Extract dimensions and raw bytes
    let (width, height) = img.dimensions();
    let mut rgba_data: Vec<u8> = img.into_raw();
    debug!(
        "Loaded image: {}x{} ({} bytes)",
        width,
        height,
        rgba_data.len()
    );

    // Read params file content
    let params = std::fs::read_to_string(&args.params)
        .with_context(|| format!("Failed to read params file: {}", args.params.display()))?;

    // Build plugin library path
    let library_name = plugin_loader::library_filename(&args.plugin);
    let plugin_library_path = args.plugin_path.join(&library_name);

    // Call plugin to process image
    plugin_loader::process(&plugin_library_path, width, height, &mut rgba_data, &params)?;

    // Reconstruct image from raw bytes
    let output_img = RgbaImage::from_raw(width, height, rgba_data)
        .expect("Buffer size mismatch - plugin must not change buffer size");

    // Save output image
    output_img
        .save(&args.output)
        .with_context(|| format!("Failed to save image: {}", args.output.display()))?;

    info!("Saved image to: {}", args.output.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_parse_all_arguments() {
        let args = Args::try_parse_from([
            "image_processor",
            "--input",
            "test_images/sample.png",
            "--output",
            "output.png",
            "--plugin",
            "mirror_plugin",
            "--params",
            "params.json",
            "--plugin-path",
            "/custom/path",
        ])
        .expect("should parse all arguments");

        assert_eq!(args.input, PathBuf::from("test_images/sample.png"));
        assert_eq!(args.output, PathBuf::from("output.png"));
        assert_eq!(args.plugin, "mirror_plugin");
        assert_eq!(args.params, PathBuf::from("params.json"));
        assert_eq!(args.plugin_path, PathBuf::from("/custom/path"));
    }

    #[test]
    fn test_args_plugin_path_default_value() {
        let args = Args::try_parse_from([
            "image_processor",
            "--input",
            "input.png",
            "--output",
            "output.png",
            "--plugin",
            "blur_plugin",
            "--params",
            "config.json",
        ])
        .expect("should parse with default plugin-path");

        assert_eq!(args.plugin_path, PathBuf::from("target/debug"));
    }

    #[test]
    fn test_args_missing_input_fails() {
        let result = Args::try_parse_from([
            "image_processor",
            "--output",
            "output.png",
            "--plugin",
            "mirror_plugin",
            "--params",
            "params.json",
        ]);

        assert!(result.is_err(), "should fail when --input is missing");
    }

    #[test]
    fn test_args_missing_output_fails() {
        let result = Args::try_parse_from([
            "image_processor",
            "--input",
            "input.png",
            "--plugin",
            "mirror_plugin",
            "--params",
            "params.json",
        ]);

        assert!(result.is_err(), "should fail when --output is missing");
    }

    #[test]
    fn test_args_missing_plugin_fails() {
        let result = Args::try_parse_from([
            "image_processor",
            "--input",
            "input.png",
            "--output",
            "output.png",
            "--params",
            "params.json",
        ]);

        assert!(result.is_err(), "should fail when --plugin is missing");
    }

    #[test]
    fn test_args_missing_params_fails() {
        let result = Args::try_parse_from([
            "image_processor",
            "--input",
            "input.png",
            "--output",
            "output.png",
            "--plugin",
            "mirror_plugin",
        ]);

        assert!(result.is_err(), "should fail when --params is missing");
    }

    #[test]
    fn test_args_paths_preserve_structure() {
        let args = Args::try_parse_from([
            "image_processor",
            "--input",
            "nested/dir/image.png",
            "--output",
            "../relative/output.png",
            "--plugin",
            "test_plugin",
            "--params",
            "./config/params.json",
        ])
        .expect("should parse paths with various structures");

        assert_eq!(args.input, PathBuf::from("nested/dir/image.png"));
        assert_eq!(args.output, PathBuf::from("../relative/output.png"));
        assert_eq!(args.params, PathBuf::from("./config/params.json"));
    }

    #[test]
    fn test_args_plugin_name_accepts_various_formats() {
        let args = Args::try_parse_from([
            "image_processor",
            "--input",
            "in.png",
            "--output",
            "out.png",
            "--plugin",
            "my-custom_plugin123",
            "--params",
            "p.json",
        ])
        .expect("should accept plugin name with hyphens, underscores, and numbers");

        assert_eq!(args.plugin, "my-custom_plugin123");
    }
}
