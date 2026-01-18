use std::ffi::{CString, c_char};
use std::path::Path;

use anyhow::{Context, Result};
use libloading::Library;
use log::{debug, info};

type ProcessImageFn = unsafe extern "C" fn(u32, u32, *mut u8, *const c_char);

/// Returns the platform-specific library filename for a plugin.
pub(crate) fn library_filename(plugin_name: &str) -> String {
    if cfg!(target_os = "macos") {
        format!("lib{}.dylib", plugin_name)
    } else if cfg!(target_os = "linux") {
        format!("lib{}.so", plugin_name)
    } else if cfg!(target_os = "windows") {
        format!("{}.dll", plugin_name)
    } else {
        // Fallback to Linux-style naming for unknown platforms
        format!("lib{}.so", plugin_name)
    }
}

/// Loads a plugin from the given path and processes the image data.
///
/// # Arguments
/// * `plugin_path` - Full path to the plugin library file
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `rgba_data` - Mutable slice of RGBA pixel data (must be width * height * 4 bytes)
/// * `params` - JSON parameters string to pass to the plugin
pub fn process(
    plugin_path: &Path,
    width: u32,
    height: u32,
    rgba_data: &mut [u8],
    params: &str,
) -> Result<()> {
    debug!(
        "plugin_loader::process called with path={}, dimensions={}x{}, params={}",
        plugin_path.display(),
        width,
        height,
        params
    );

    info!("Loading plugin from: {}", plugin_path.display());

    // SAFETY: The library path is provided by the user and we trust the library to be a valid plugin
    let lib = unsafe { Library::new(plugin_path) }
        .with_context(|| format!("Failed to load plugin library: {}", plugin_path.display()))?;

    // SAFETY: The symbol name is null-terminated and we trust the library exports this symbol with the correct signature
    let process_image_fn: libloading::Symbol<ProcessImageFn> =
        unsafe { lib.get(b"process_image\0") }
            .with_context(|| "Failed to find process_image symbol")?;

    let c_params = CString::new(params).with_context(|| "Invalid params string")?;

    // SAFETY: The rgba_data buffer is valid for width*height*4 bytes, c_params is a valid CString pointer, and the library is loaded
    unsafe {
        process_image_fn(width, height, rgba_data.as_mut_ptr(), c_params.as_ptr());
    }

    info!("Plugin execution complete");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_filename_current_platform() {
        let name = library_filename("mirror_plugin");

        #[cfg(target_os = "macos")]
        assert_eq!(name, "libmirror_plugin.dylib");

        #[cfg(target_os = "linux")]
        assert_eq!(name, "libmirror_plugin.so");

        #[cfg(target_os = "windows")]
        assert_eq!(name, "mirror_plugin.dll");
    }

    #[test]
    fn test_process_missing_library_returns_error() {
        let path = std::path::Path::new("/nonexistent/path/libfake.dylib");
        let mut data = vec![0u8; 16]; // 2x2 RGBA
        let result = process(path, 2, 2, &mut data, "{}");

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Failed to load plugin library"));
    }

    #[test]
    #[ignore] // Run with: cargo test -p image_processor -- --ignored
    fn test_process_with_real_plugin() {
        // Build the plugin library path
        let lib_name = library_filename("mirror_plugin");
        let plugin_path = std::path::PathBuf::from("../target/debug").join(&lib_name);

        // Create a small test image (2x2 pixels, RGBA)
        let mut data = vec![
            255, 0, 0, 255, // Red pixel
            0, 255, 0, 255, // Green pixel
            0, 0, 255, 255, // Blue pixel
            255, 255, 0, 255, // Yellow pixel
        ];

        let result = process(&plugin_path, 2, 2, &mut data, r#"{"horizontal": true}"#);

        // Check the plugin executed without error
        assert!(result.is_ok(), "Expected success, got: {:?}", result);
    }
}
