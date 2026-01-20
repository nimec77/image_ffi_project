use log::error;
use serde::Deserialize;
use std::ffi::{CStr, c_char};

#[derive(Deserialize)]
struct Params {
    #[serde(default = "default_radius")]
    radius: u32,
    #[serde(default = "default_iterations")]
    iterations: u32,
}

fn default_radius() -> u32 {
    1
}

fn default_iterations() -> u32 {
    1
}

/// Processes an image by applying a blur effect.
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
) {
    // SAFETY: params is a valid null-terminated C string passed by the host.
    // The plugin loader guarantees this pointer is valid for the duration of this call.
    let params_str = unsafe { CStr::from_ptr(params) }.to_str().unwrap_or("");

    let params: Params = match serde_json::from_str(params_str) {
        Ok(p) => p,
        Err(e) => {
            error!("blur_plugin: failed to parse params JSON: {}", e);
            return;
        }
    };

    // Early return if no blur needed
    if params.radius == 0 || params.iterations == 0 {
        return;
    }

    let width = width as usize;
    let height = height as usize;
    let len = width * height * 4;
    let radius = params.radius as i32;

    // SAFETY: rgba_data is a valid pointer to a buffer of exactly width * height * 4 bytes,
    // owned by the host. The plugin loader guarantees this buffer is valid and properly
    // aligned for the duration of this call. We only access indices within bounds.
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, len) };

    // Allocate temporary buffer for intermediate results
    let mut temp_buffer = vec![0u8; len];

    // Apply blur for the specified number of iterations
    for _ in 0..params.iterations {
        // For each pixel, compute weighted average of neighbors within radius
        for cy in 0..height {
            for cx in 0..width {
                let mut weight_sum = 0.0_f64;
                let mut color_sum = [0.0_f64; 4]; // R, G, B, A

                // Iterate over neighbors within radius
                for dy in -radius..=radius {
                    for dx in -radius..=radius {
                        let nx = cx as i32 + dx;
                        let ny = cy as i32 + dy;

                        // Check bounds
                        if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                            let nx = nx as usize;
                            let ny = ny as usize;

                            // Calculate Euclidean distance
                            let distance = ((dx * dx + dy * dy) as f64).sqrt();
                            let weight = 1.0 / (distance + 1.0);

                            weight_sum += weight;

                            // Accumulate weighted color values
                            let neighbor_idx = (ny * width + nx) * 4;
                            for channel in 0..4 {
                                color_sum[channel] += weight * data[neighbor_idx + channel] as f64;
                            }
                        }
                    }
                }

                // Store weighted average in temp buffer
                let pixel_idx = (cy * width + cx) * 4;
                for channel in 0..4 {
                    temp_buffer[pixel_idx + channel] =
                        (color_sum[channel] / weight_sum).round() as u8;
                }
            }
        }

        // Copy temp buffer back to original data
        data.copy_from_slice(&temp_buffer);
    }
}

#[cfg(test)]
#[allow(clippy::identity_op)] // Allow (row * width + col) for readability
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_params_full_json() {
        let json = r#"{"radius": 5, "iterations": 3}"#;
        let params: Params = serde_json::from_str(json).expect("valid JSON");
        assert_eq!(params.radius, 5);
        assert_eq!(params.iterations, 3);
    }

    #[test]
    fn test_params_empty_json() {
        let json = "{}";
        let params: Params = serde_json::from_str(json).expect("valid JSON");
        assert_eq!(params.radius, 1);
        assert_eq!(params.iterations, 1);
    }

    #[test]
    fn test_params_partial_json_radius_only() {
        let json = r#"{"radius": 5}"#;
        let params: Params = serde_json::from_str(json).expect("valid JSON");
        assert_eq!(params.radius, 5);
        assert_eq!(params.iterations, 1);
    }

    #[test]
    fn test_params_partial_json_iterations_only() {
        let json = r#"{"iterations": 3}"#;
        let params: Params = serde_json::from_str(json).expect("valid JSON");
        assert_eq!(params.radius, 1);
        assert_eq!(params.iterations, 3);
    }

    fn blur_image(data: &mut [u8], width: u32, height: u32, params_json: &str) {
        let c_params = CString::new(params_json).expect("CString creation failed");
        // SAFETY: data is a valid slice with length >= width * height * 4,
        // and c_params is a valid null-terminated C string.
        unsafe { process_image(width, height, data.as_mut_ptr(), c_params.as_ptr()) };
    }

    fn create_4x4_sharp_edge() -> Vec<u8> {
        let mut data = vec![0u8; 4 * 4 * 4];
        for y in 0..4 {
            for x in 0..4 {
                let idx = (y * 4 + x) * 4;
                let value = if x < 2 { 0 } else { 255 };
                data[idx] = value;
                data[idx + 1] = value;
                data[idx + 2] = value;
                data[idx + 3] = 255;
            }
        }
        data
    }

    #[test]
    fn test_basic_blur() {
        let mut data = vec![
            0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255, 255, 255, 255, 255, 0, 0, 0,
            255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255,
        ];
        let original = data.clone();

        blur_image(&mut data, 3, 3, r#"{"radius": 1, "iterations": 1}"#);

        assert_ne!(data, original, "Blur should modify the image");
        let center_idx = (1 * 3 + 1) * 4;
        assert!(
            data[center_idx] < 255,
            "Center pixel should be darker after blur"
        );
    }

    #[test]
    fn test_blur_smoothing() {
        let mut data = create_4x4_sharp_edge();
        let original = data.clone();

        blur_image(&mut data, 4, 4, r#"{"radius": 1, "iterations": 1}"#);

        let left_edge_idx = (1 * 4 + 1) * 4;
        assert!(
            data[left_edge_idx] > 0,
            "Left edge pixel should be lighter after blur"
        );

        let right_edge_idx = (1 * 4 + 2) * 4;
        assert!(
            data[right_edge_idx] < 255,
            "Right edge pixel should be darker after blur"
        );

        let original_diff =
            (original[right_edge_idx] as i32 - original[left_edge_idx] as i32).abs();
        let new_diff = (data[right_edge_idx] as i32 - data[left_edge_idx] as i32).abs();
        assert!(
            new_diff < original_diff,
            "Edge should be smoother after blur"
        );
    }

    #[test]
    fn test_zero_radius() {
        let mut data = create_4x4_sharp_edge();
        let original = data.clone();

        blur_image(&mut data, 4, 4, r#"{"radius": 0, "iterations": 1}"#);

        assert_eq!(data, original, "Image should not be modified when radius=0");
    }

    #[test]
    fn test_zero_iterations() {
        let mut data = create_4x4_sharp_edge();
        let original = data.clone();

        blur_image(&mut data, 4, 4, r#"{"radius": 1, "iterations": 0}"#);

        assert_eq!(
            data, original,
            "Image should not be modified when iterations=0"
        );
    }

    #[test]
    fn test_multiple_iterations() {
        let mut data_single = create_4x4_sharp_edge();
        let mut data_multiple = create_4x4_sharp_edge();

        blur_image(&mut data_single, 4, 4, r#"{"radius": 1, "iterations": 1}"#);
        blur_image(
            &mut data_multiple,
            4,
            4,
            r#"{"radius": 1, "iterations": 3}"#,
        );

        let edge_idx = (1 * 4 + 1) * 4;
        assert!(
            data_multiple[edge_idx] > data_single[edge_idx],
            "More iterations should increase blur effect"
        );

        assert_ne!(
            data_single, data_multiple,
            "Different iteration counts should produce different results"
        );
    }

    #[test]
    fn test_1x1_image() {
        let mut data = vec![128u8, 64, 32, 255];
        let original = data.clone();

        blur_image(&mut data, 1, 1, r#"{"radius": 1, "iterations": 1}"#);

        assert_eq!(data, original, "Single pixel image should remain unchanged");
    }

    #[test]
    fn test_invalid_json() {
        let mut data = create_4x4_sharp_edge();
        let original = data.clone();

        blur_image(&mut data, 4, 4, "not valid json {{{");

        assert_eq!(
            data, original,
            "Image should not be modified on invalid JSON"
        );
    }

    #[test]
    fn test_empty_json() {
        let mut data = create_4x4_sharp_edge();
        let original = data.clone();

        blur_image(&mut data, 4, 4, "{}");

        assert_ne!(
            data, original,
            "Empty JSON should apply defaults and blur the image"
        );
    }
}
