use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

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
    let args = Args::parse();

    println!("Input: {:?}", args.input);
    println!("Output: {:?}", args.output);
    println!("Plugin: {}", args.plugin);
    println!("Params: {:?}", args.params);
    println!("Plugin path: {:?}", args.plugin_path);

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
