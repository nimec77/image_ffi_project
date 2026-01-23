use log::error;
use serde::Deserialize;
use std::ffi::{CStr, c_char};

/// Error codes returned by the mirror plugin.
#[repr(i32)]
pub enum MirrorError {
    Success = 0,
    ParseError = -1,
    SizeOverflow = -2,
}

#[derive(Deserialize)]
struct Params {
    #[serde(default)]
    horizontal: bool,
    #[serde(default)]
    vertical: bool,
}

/// Processes an image by applying horizontal and/or vertical flip transformations.
///
/// # Safety
///
/// The caller must ensure:
/// - `rgba_data` is a valid pointer to a buffer of exactly `width * height * 4` bytes
/// - `params` is a valid null-terminated C string
/// - The buffer remains valid for the duration of this call
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) -> i32 {
    // SAFETY: params is a valid null-terminated C string passed by the host.
    // The plugin loader guarantees this pointer is valid for the duration of this call.
    let params_str = unsafe { CStr::from_ptr(params) }.to_str().unwrap_or("");

    // Deserialize JSON into Params
    let params: Params = match serde_json::from_str(params_str) {
        Ok(p) => p,
        Err(e) => {
            error!("mirror_plugin: failed to parse params JSON: {}", e);
            return MirrorError::ParseError as i32;
        }
    };

    // Early return if no flip requested
    if !params.horizontal && !params.vertical {
        return MirrorError::Success as i32;
    }

    let width_usize = width as usize;
    let height_usize = height as usize;
    let len = match width_usize
        .checked_mul(height_usize)
        .and_then(|n| n.checked_mul(4))
    {
        Some(len) => len,
        None => {
            error!("mirror_plugin: size overflow calculating buffer length");
            return MirrorError::SizeOverflow as i32;
        }
    };

    // SAFETY: rgba_data is a valid pointer to a buffer of exactly width * height * 4 bytes,
    // owned by the host. The plugin loader guarantees this buffer is valid and properly
    // aligned for the duration of this call. We only access indices within bounds.
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, len) };

    // Horizontal flip: swap pixels within each row
    if params.horizontal {
        for y in 0..height_usize {
            for x in 0..width_usize / 2 {
                let left_idx = (y * width_usize + x) * 4;
                let right_idx = (y * width_usize + (width_usize - 1 - x)) * 4;
                // Swap 4 bytes (RGBA) at a time
                for i in 0..4 {
                    data.swap(left_idx + i, right_idx + i);
                }
            }
        }
    }

    // Vertical flip: swap rows
    if params.vertical {
        let row_bytes = match width_usize.checked_mul(4) {
            Some(rb) => rb,
            None => {
                error!("mirror_plugin: size overflow calculating row bytes");
                return MirrorError::SizeOverflow as i32;
            }
        };
        for y in 0..height_usize / 2 {
            let top_start = y * row_bytes;
            let bottom_start = (height_usize - 1 - y) * row_bytes;
            // Swap each byte in the row
            for i in 0..row_bytes {
                data.swap(top_start + i, bottom_start + i);
            }
        }
    }

    MirrorError::Success as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    /// Helper function to call process_image with test data.
    /// Returns the error code from the plugin.
    fn call_process_image(width: u32, height: u32, data: &mut [u8], params_json: &str) -> i32 {
        let params = CString::new(params_json).expect("CString creation failed");
        // SAFETY: data is a valid slice with length >= width * height * 4,
        // and params is a valid null-terminated C string.
        unsafe { process_image(width, height, data.as_mut_ptr(), params.as_ptr()) }
    }

    /// Creates a 4x4 test image where each pixel has a unique value based on position.
    /// Pixel at (x, y) has RGBA = (x, y, x+y, 255)
    fn create_4x4_test_image() -> Vec<u8> {
        let mut data = vec![0u8; 4 * 4 * 4]; // 4x4 pixels, 4 bytes each
        for y in 0..4 {
            for x in 0..4 {
                let idx = (y * 4 + x) * 4;
                data[idx] = x as u8; // R = x
                data[idx + 1] = y as u8; // G = y
                data[idx + 2] = (x + y) as u8; // B = x + y
                data[idx + 3] = 255; // A = 255
            }
        }
        data
    }

    /// Creates a 3x3 test image for odd dimension testing.
    /// Pixel at (x, y) has RGBA = (x, y, x+y, 255)
    fn create_3x3_test_image() -> Vec<u8> {
        let mut data = vec![0u8; 3 * 3 * 4]; // 3x3 pixels, 4 bytes each
        for y in 0..3 {
            for x in 0..3 {
                let idx = (y * 3 + x) * 4;
                data[idx] = x as u8; // R = x
                data[idx + 1] = y as u8; // G = y
                data[idx + 2] = (x + y) as u8; // B = x + y
                data[idx + 3] = 255; // A = 255
            }
        }
        data
    }

    /// Gets the pixel RGBA values at position (x, y) for a given width
    fn get_pixel(data: &[u8], width: usize, x: usize, y: usize) -> (u8, u8, u8, u8) {
        let idx = (y * width + x) * 4;
        (data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
    }

    #[test]
    fn test_horizontal_flip() {
        let mut data = create_4x4_test_image();
        call_process_image(
            4,
            4,
            &mut data,
            r#"{"horizontal": true, "vertical": false}"#,
        );

        // After horizontal flip, pixel at (x, y) should have values from original (3-x, y)
        // So new (0, 0) should have original (3, 0) values: R=3, G=0, B=3, A=255
        assert_eq!(get_pixel(&data, 4, 0, 0), (3, 0, 3, 255));
        assert_eq!(get_pixel(&data, 4, 1, 0), (2, 0, 2, 255));
        assert_eq!(get_pixel(&data, 4, 2, 0), (1, 0, 1, 255));
        assert_eq!(get_pixel(&data, 4, 3, 0), (0, 0, 0, 255));

        // Check another row
        assert_eq!(get_pixel(&data, 4, 0, 2), (3, 2, 5, 255));
        assert_eq!(get_pixel(&data, 4, 3, 2), (0, 2, 2, 255));
    }

    #[test]
    fn test_vertical_flip() {
        let mut data = create_4x4_test_image();
        call_process_image(
            4,
            4,
            &mut data,
            r#"{"horizontal": false, "vertical": true}"#,
        );

        // After vertical flip, pixel at (x, y) should have values from original (x, 3-y)
        // So new (0, 0) should have original (0, 3) values: R=0, G=3, B=3, A=255
        assert_eq!(get_pixel(&data, 4, 0, 0), (0, 3, 3, 255));
        assert_eq!(get_pixel(&data, 4, 0, 1), (0, 2, 2, 255));
        assert_eq!(get_pixel(&data, 4, 0, 2), (0, 1, 1, 255));
        assert_eq!(get_pixel(&data, 4, 0, 3), (0, 0, 0, 255));

        // Check another column
        assert_eq!(get_pixel(&data, 4, 2, 0), (2, 3, 5, 255));
        assert_eq!(get_pixel(&data, 4, 2, 3), (2, 0, 2, 255));
    }

    #[test]
    fn test_combined_flip() {
        let mut data = create_4x4_test_image();
        call_process_image(4, 4, &mut data, r#"{"horizontal": true, "vertical": true}"#);

        // Combined flip is equivalent to 180-degree rotation
        // Pixel at (x, y) should have values from original (3-x, 3-y)
        // So new (0, 0) should have original (3, 3) values: R=3, G=3, B=6, A=255
        assert_eq!(get_pixel(&data, 4, 0, 0), (3, 3, 6, 255));
        assert_eq!(get_pixel(&data, 4, 3, 3), (0, 0, 0, 255));
        assert_eq!(get_pixel(&data, 4, 1, 1), (2, 2, 4, 255));
        assert_eq!(get_pixel(&data, 4, 2, 2), (1, 1, 2, 255));
    }

    #[test]
    fn test_no_flip() {
        let mut data = create_4x4_test_image();
        let original = data.clone();
        call_process_image(
            4,
            4,
            &mut data,
            r#"{"horizontal": false, "vertical": false}"#,
        );

        // Image should remain unchanged
        assert_eq!(data, original);
    }

    #[test]
    fn test_1x1_image() {
        // Single pixel: RGBA = (42, 128, 200, 255)
        let mut data = vec![42u8, 128, 200, 255];
        let original = data.clone();

        // Horizontal flip on 1x1 should leave image unchanged
        call_process_image(
            1,
            1,
            &mut data,
            r#"{"horizontal": true, "vertical": false}"#,
        );
        assert_eq!(data, original);

        // Vertical flip on 1x1 should leave image unchanged
        call_process_image(
            1,
            1,
            &mut data,
            r#"{"horizontal": false, "vertical": true}"#,
        );
        assert_eq!(data, original);

        // Combined flip on 1x1 should leave image unchanged
        call_process_image(1, 1, &mut data, r#"{"horizontal": true, "vertical": true}"#);
        assert_eq!(data, original);
    }

    #[test]
    fn test_odd_dimensions_horizontal() {
        let mut data = create_3x3_test_image();
        call_process_image(
            3,
            3,
            &mut data,
            r#"{"horizontal": true, "vertical": false}"#,
        );

        // After horizontal flip, pixel at (x, y) should have values from original (2-x, y)
        // Middle column (x=1) should stay in place
        assert_eq!(get_pixel(&data, 3, 0, 0), (2, 0, 2, 255)); // was (2, 0)
        assert_eq!(get_pixel(&data, 3, 1, 0), (1, 0, 1, 255)); // was (1, 0) - middle stays
        assert_eq!(get_pixel(&data, 3, 2, 0), (0, 0, 0, 255)); // was (0, 0)

        // Middle row
        assert_eq!(get_pixel(&data, 3, 0, 1), (2, 1, 3, 255));
        assert_eq!(get_pixel(&data, 3, 1, 1), (1, 1, 2, 255)); // center pixel unchanged
        assert_eq!(get_pixel(&data, 3, 2, 1), (0, 1, 1, 255));
    }

    #[test]
    fn test_odd_dimensions_vertical() {
        let mut data = create_3x3_test_image();
        call_process_image(
            3,
            3,
            &mut data,
            r#"{"horizontal": false, "vertical": true}"#,
        );

        // After vertical flip, pixel at (x, y) should have values from original (x, 2-y)
        // Middle row (y=1) should stay in place
        assert_eq!(get_pixel(&data, 3, 0, 0), (0, 2, 2, 255)); // was (0, 2)
        assert_eq!(get_pixel(&data, 3, 0, 1), (0, 1, 1, 255)); // was (0, 1) - middle stays
        assert_eq!(get_pixel(&data, 3, 0, 2), (0, 0, 0, 255)); // was (0, 0)

        // Middle column
        assert_eq!(get_pixel(&data, 3, 1, 0), (1, 2, 3, 255));
        assert_eq!(get_pixel(&data, 3, 1, 1), (1, 1, 2, 255)); // center pixel unchanged
        assert_eq!(get_pixel(&data, 3, 1, 2), (1, 0, 1, 255));
    }

    #[test]
    fn test_invalid_json() {
        let mut data = create_4x4_test_image();
        let original = data.clone();

        // Invalid JSON should result in ParseError without modifying the image
        let result = call_process_image(4, 4, &mut data, "not valid json {{{");

        assert_eq!(result, MirrorError::ParseError as i32);
        assert_eq!(data, original);
    }

    #[test]
    fn test_empty_json() {
        let mut data = create_4x4_test_image();
        let original = data.clone();

        // Empty JSON object should use default values (both false)
        call_process_image(4, 4, &mut data, "{}");

        assert_eq!(data, original);
    }

    #[test]
    fn test_partial_params() {
        let mut data = create_4x4_test_image();

        // Only horizontal specified, vertical defaults to false
        call_process_image(4, 4, &mut data, r#"{"horizontal": true}"#);

        // Should be horizontally flipped
        assert_eq!(get_pixel(&data, 4, 0, 0), (3, 0, 3, 255));
        assert_eq!(get_pixel(&data, 4, 3, 0), (0, 0, 0, 255));
    }

    #[test]
    fn test_returns_success_on_valid_params() {
        let mut data = create_4x4_test_image();
        let result = call_process_image(4, 4, &mut data, r#"{"horizontal": true}"#);
        assert_eq!(result, MirrorError::Success as i32);
    }

    #[test]
    fn test_returns_success_when_no_flip() {
        let mut data = create_4x4_test_image();
        let result = call_process_image(
            4,
            4,
            &mut data,
            r#"{"horizontal": false, "vertical": false}"#,
        );
        assert_eq!(result, MirrorError::Success as i32);
    }
}
