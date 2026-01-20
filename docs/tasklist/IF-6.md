# IF-6: Blur Plugin

Status: IMPLEMENT_STEP_OK

Context: PRD `docs/prd/IF-6.prd.md`; plan `docs/plan/IF-6.md`

---

## Tasks

- [x] **Task 1: Define Params struct with serde defaults**
  - Add `Params` struct to `blur_plugin/src/lib.rs` with `radius: u32` and `iterations: u32` fields
  - Add `#[serde(default)]` with helper functions for defaults (radius=1, iterations=1)
  - **AC1**: Struct compiles and derives `Deserialize`
  - **AC2**: Empty JSON `{}` deserializes to `Params { radius: 1, iterations: 1 }`

- [x] **Task 2: Implement JSON parameter parsing in process_image**
  - Parse C string params to Rust `&str` using `CStr::from_ptr`
  - Deserialize JSON into `Params` struct using `serde_json::from_str`
  - Handle errors gracefully: log error and return early without modification
  - **AC1**: Valid JSON `{"radius": 3, "iterations": 2}` parses correctly
  - **AC2**: Invalid JSON logs error and returns without panicking

- [x] **Task 3: Implement weighted average blur algorithm**
  - Convert raw pointer to mutable slice for RGBA data
  - Allocate temporary buffer for intermediate results
  - For each pixel, compute weighted average of neighbors within radius
  - Use weight formula: `1.0 / (distance + 1.0)` where distance is Euclidean
  - Handle edge pixels by only averaging valid neighbors within bounds
  - Copy temp buffer back to original after processing all pixels
  - Add `// SAFETY:` comments to all unsafe blocks
  - **AC1**: Blur with radius=1 modifies pixel values (output differs from input)
  - **AC2**: Sharp edge in test image becomes smoother after blur

- [x] **Task 4: Support multiple iterations**
  - Wrap blur algorithm in loop for specified number of iterations
  - Early return if radius=0 or iterations=0 (no-op)
  - **AC1**: `iterations=0` leaves image unchanged
  - **AC2**: `iterations=3` produces stronger blur than `iterations=1` (more averaging)

- [x] **Task 5: Create test parameters file**
  - Create `test_images/blur_params.json` with `{"radius": 3, "iterations": 1}`
  - **AC1**: File exists and contains valid JSON
  - **AC2**: CLI can use file: `cargo run -p image_processor -- --input test_images/sample.png --output /tmp/blur_out.png --plugin blur_plugin --params test_images/blur_params.json` succeeds

- [x] **Task 6: Add unit tests**
  - `test_basic_blur`: Verify blur modifies pixels (not identical to input)
  - `test_blur_smoothing`: Verify sharp edge becomes smoother
  - `test_zero_radius`: Verify no modification when radius=0
  - `test_zero_iterations`: Verify no modification when iterations=0
  - `test_multiple_iterations`: Verify stronger blur with more iterations
  - `test_1x1_image`: Verify single pixel image handled correctly
  - `test_invalid_json`: Verify early return without modification on invalid JSON
  - `test_empty_json`: Verify defaults applied (radius=1, iterations=1)
  - **AC1**: `cargo test -p blur_plugin` passes with all tests green
  - **AC2**: Tests cover edge cases: empty image, 1x1 image, zero parameters

- [x] **Task 7: Verify code quality**
  - Run `cargo fmt` to format code
  - Run `cargo clippy -- -D warnings` and fix any warnings
  - Run `cargo build` and ensure no warnings
  - Run `cargo test` to ensure all tests pass
  - **AC1**: `cargo clippy -- -D warnings` exits with code 0
  - **AC2**: `cargo build` produces no warnings
  - **AC3**: `cargo test` exits with code 0
