use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Returns the path to the compiled image_processor binary.
fn get_binary_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("failed to get workspace root for binary path")
        .join("target")
        .join("debug")
        .join("image_processor")
}

/// Returns the path to the directory containing plugin libraries (.dylib/.so/.dll).
fn get_plugin_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("failed to get workspace root for plugin directory")
        .join("target")
        .join("debug")
}

/// Returns the path to the test images directory.
fn get_test_images_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("failed to get workspace root for test images directory")
        .join("test_images")
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;

    #[test]
    fn test_helper_paths_exist() {
        // Verify the helper functions return valid paths
        let binary_path = get_binary_path();
        let plugin_dir = get_plugin_dir();
        let test_images_dir = get_test_images_dir();

        // These paths should be constructible (actual file existence depends on build state)
        assert!(binary_path.to_string_lossy().contains("image_processor"));
        assert!(plugin_dir.to_string_lossy().contains("debug"));
        assert!(test_images_dir.to_string_lossy().contains("test_images"));
    }

    #[test]
    fn test_mirror_plugin_horizontal_flip() {
        let binary_path = get_binary_path();
        let plugin_dir = get_plugin_dir();
        let test_images_dir = get_test_images_dir();

        let input_path = test_images_dir.join("sample.png");
        let params_path = test_images_dir.join("mirror_params.json");

        // Create temp directory for output
        let temp_dir = TempDir::new().expect("failed to create temp directory");
        let output_path = temp_dir.path().join("output.png");

        // Execute the image_processor binary
        let output = Command::new(&binary_path)
            .arg("--input")
            .arg(&input_path)
            .arg("--output")
            .arg(&output_path)
            .arg("--plugin")
            .arg("mirror_plugin")
            .arg("--plugin-path")
            .arg(&plugin_dir)
            .arg("--params")
            .arg(&params_path)
            .output()
            .expect("failed to execute image_processor binary");

        // Verify: process exits 0
        assert!(
            output.status.success(),
            "image_processor failed with stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        // Verify: output file exists
        assert!(
            output_path.exists(),
            "output file does not exist at {:?}",
            output_path
        );

        // Verify: output image dimensions match input
        let input_image = image::open(&input_path).expect("failed to open input image");
        let output_image = image::open(&output_path).expect("failed to open output image");

        assert_eq!(
            input_image.dimensions(),
            output_image.dimensions(),
            "output image dimensions do not match input"
        );
    }

    #[test]
    fn test_blur_plugin_workflow() {
        let binary_path = get_binary_path();
        let plugin_dir = get_plugin_dir();
        let test_images_dir = get_test_images_dir();

        let input_path = test_images_dir.join("sample.png");
        let params_path = test_images_dir.join("blur_params.json");

        // Create temp directory for output
        let temp_dir = TempDir::new().expect("failed to create temp directory");
        let output_path = temp_dir.path().join("output.png");

        // Execute the image_processor binary
        let output = Command::new(&binary_path)
            .arg("--input")
            .arg(&input_path)
            .arg("--output")
            .arg(&output_path)
            .arg("--plugin")
            .arg("blur_plugin")
            .arg("--plugin-path")
            .arg(&plugin_dir)
            .arg("--params")
            .arg(&params_path)
            .output()
            .expect("failed to execute image_processor binary");

        // Verify: process exits 0
        assert!(
            output.status.success(),
            "image_processor failed with stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        // Verify: output file exists
        assert!(
            output_path.exists(),
            "output file does not exist at {:?}",
            output_path
        );

        // Verify: output image dimensions match input
        let input_image = image::open(&input_path).expect("failed to open input image");
        let output_image = image::open(&output_path).expect("failed to open output image");

        assert_eq!(
            input_image.dimensions(),
            output_image.dimensions(),
            "output image dimensions do not match input"
        );
    }

    #[test]
    fn test_error_nonexistent_input() {
        let binary_path = get_binary_path();
        let plugin_dir = get_plugin_dir();
        let test_images_dir = get_test_images_dir();

        let input_path = test_images_dir.join("nonexistent.png");
        let params_path = test_images_dir.join("mirror_params.json");

        let temp_dir = TempDir::new().expect("failed to create temp directory");
        let output_path = temp_dir.path().join("output.png");

        let output = Command::new(&binary_path)
            .arg("--input")
            .arg(&input_path)
            .arg("--output")
            .arg(&output_path)
            .arg("--plugin")
            .arg("mirror_plugin")
            .arg("--plugin-path")
            .arg(&plugin_dir)
            .arg("--params")
            .arg(&params_path)
            .output()
            .expect("failed to execute image_processor binary");

        // Verify: process fails (non-zero exit)
        assert!(
            !output.status.success(),
            "image_processor should fail with nonexistent input"
        );

        // Verify: stderr contains meaningful error message
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("Failed to load image") || stderr.contains("nonexistent"),
            "stderr should mention the failed image load: {}",
            stderr
        );
    }

    #[test]
    fn test_error_nonexistent_plugin() {
        let binary_path = get_binary_path();
        let plugin_dir = get_plugin_dir();
        let test_images_dir = get_test_images_dir();

        let input_path = test_images_dir.join("sample.png");
        let params_path = test_images_dir.join("mirror_params.json");

        let temp_dir = TempDir::new().expect("failed to create temp directory");
        let output_path = temp_dir.path().join("output.png");

        let output = Command::new(&binary_path)
            .arg("--input")
            .arg(&input_path)
            .arg("--output")
            .arg(&output_path)
            .arg("--plugin")
            .arg("nonexistent_plugin")
            .arg("--plugin-path")
            .arg(&plugin_dir)
            .arg("--params")
            .arg(&params_path)
            .output()
            .expect("failed to execute image_processor binary");

        // Verify: process fails (non-zero exit)
        assert!(
            !output.status.success(),
            "image_processor should fail with nonexistent plugin"
        );

        // Verify: stderr contains meaningful error message
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("Failed to load plugin") || stderr.contains("nonexistent_plugin"),
            "stderr should mention the failed plugin load: {}",
            stderr
        );
    }

    #[test]
    fn test_error_invalid_params() {
        let binary_path = get_binary_path();
        let plugin_dir = get_plugin_dir();
        let test_images_dir = get_test_images_dir();

        let input_path = test_images_dir.join("sample.png");
        let params_path = test_images_dir.join("nonexistent_params.json");

        let temp_dir = TempDir::new().expect("failed to create temp directory");
        let output_path = temp_dir.path().join("output.png");

        let output = Command::new(&binary_path)
            .arg("--input")
            .arg(&input_path)
            .arg("--output")
            .arg(&output_path)
            .arg("--plugin")
            .arg("mirror_plugin")
            .arg("--plugin-path")
            .arg(&plugin_dir)
            .arg("--params")
            .arg(&params_path)
            .output()
            .expect("failed to execute image_processor binary");

        // Verify: process fails (non-zero exit)
        assert!(
            !output.status.success(),
            "image_processor should fail with nonexistent params file"
        );

        // Verify: stderr contains meaningful error message
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("Failed to read params") || stderr.contains("nonexistent_params"),
            "stderr should mention the failed params read: {}",
            stderr
        );
    }
}
