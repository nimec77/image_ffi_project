# IF-5: Mirror Plugin

Status: IMPLEMENT_STEP_OK

Context: PRD `docs/prd/IF-5.prd.md`; Plan `docs/plan/IF-5.md`

## Tasks

- [x] **5.1 Define Params Struct**
  - Add `Params` struct with `horizontal: bool` and `vertical: bool` fields
  - Use `#[derive(Deserialize)]` and `#[serde(default)]` for default values
  - **Acceptance Criteria**:
    - `Params` struct exists in `mirror_plugin/src/lib.rs`
    - Both fields default to `false` when omitted from JSON

- [x] **5.2 Parse JSON Parameters**
  - Convert C string params to Rust `&str` using `CStr::from_ptr`
  - Deserialize JSON into `Params` using `serde_json::from_str`
  - Handle errors gracefully (log error and return early without modifying image)
  - **Acceptance Criteria**:
    - Valid JSON `{"horizontal": true}` parses successfully
    - Invalid JSON logs error and returns without panic
    - Empty JSON `{}` uses default values (both false)

- [x] **5.3 Implement Horizontal Flip**
  - Swap pixels within each row from left edge to right edge
  - Use pixel formula: `index = (y * width + x) * 4`
  - Add `// SAFETY:` comment for unsafe block
  - **Acceptance Criteria**:
    - Horizontal flip swaps pixel at (x, y) with pixel at (width-1-x, y)
    - Unit test verifies correct pixel positions after horizontal flip

- [x] **5.4 Implement Vertical Flip**
  - Swap entire rows from top to bottom
  - Row size is `width * 4` bytes
  - Add `// SAFETY:` comment for unsafe block
  - **Acceptance Criteria**:
    - Vertical flip swaps row y with row (height-1-y)
    - Unit test verifies correct pixel positions after vertical flip

- [x] **5.5 Add Test Parameters File**
  - Create/update `test_images/mirror_params.json` with default test params
  - **Acceptance Criteria**:
    - File contains valid JSON: `{"horizontal": true, "vertical": false}`
    - Manual CLI test works: `./target/debug/image_processor --input test_images/sample.png --output output.png --plugin mirror_plugin --params test_images/mirror_params.json`

- [x] **5.6 Unit Tests for Flip Logic**
  - Test horizontal flip with small test image (e.g., 4x4)
  - Test vertical flip with small test image
  - Test combined flip (both horizontal and vertical)
  - Test edge cases: 1x1 image, odd dimensions
  - **Acceptance Criteria**:
    - `cargo test -p mirror_plugin` passes
    - Tests cover: horizontal only, vertical only, both, neither, 1x1, odd dimensions

- [x] **5.7 Integration Test**
  - Verify end-to-end workflow with mirror_plugin
  - Test that output image is correctly transformed
  - **Acceptance Criteria**:
    - `cargo build` succeeds without warnings
    - `cargo clippy -- -D warnings` passes
    - `cargo fmt --check` passes
    - CLI produces correctly mirrored output image
