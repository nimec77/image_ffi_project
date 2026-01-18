# IF-3: Image I/O

Status: IMPLEMENT_STEP_OK

Context: Implement image loading and saving functionality using the `image` crate. This extends `main.rs` to load PNG files, extract dimensions and raw RGBA data, and save results. Also initializes logging via `env_logger` and creates test assets.

Reference: `docs/prd/IF-3.prd.md`, `docs/plan/IF-3.md`

---

## Tasks

- [x] **Task 1: Add required imports to main.rs**
  - Add `use anyhow::Context;` for error context
  - Add `use image::RgbaImage;` for image reconstruction
  - Add `use log::{debug, info};` for logging macros
  - **Acceptance Criteria:**
    - `cargo check -p image_processor` compiles without errors
    - All three imports are present in `image_processor/src/main.rs`

- [x] **Task 2: Initialize logging with env_logger**
  - Add `env_logger::init();` at the start of `main()` function
  - **Acceptance Criteria:**
    - Running with `RUST_LOG=debug cargo run -- --help` shows no panic or error
    - Logger is initialized before any other logic in `main()`

- [x] **Task 3: Load PNG image and convert to RGBA8**
  - Use `image::open(&args.input)` to load the image
  - Chain `.with_context()` for error message including the path
  - Convert to RGBA8 with `.into_rgba8()`
  - **Acceptance Criteria:**
    - Loading a valid PNG file succeeds
    - Loading a non-existent file returns error containing "Failed to load image" and the file path

- [x] **Task 4: Extract dimensions and get raw bytes**
  - Call `.dimensions()` to get `(width, height)` tuple
  - Call `.into_raw()` to get `Vec<u8>` of RGBA pixel data
  - Add `debug!()` log statement showing dimensions (e.g., "Loaded image: 100x100")
  - **Acceptance Criteria:**
    - Running with `RUST_LOG=debug` logs the image dimensions
    - Buffer size equals `width * height * 4`

- [x] **Task 5: Reconstruct image and save to output path**
  - Use `RgbaImage::from_raw(width, height, rgba_data)` to reconstruct image
  - Use `.expect()` with message for the `Option` return (buffer size mismatch is programmer error)
  - Call `.save(&args.output)` with `.with_context()` for error handling
  - **Acceptance Criteria:**
    - Running `cargo run -- -i test_images/sample.png -o output.png --plugin p --params params.json` creates `output.png`
    - Error on invalid output path includes "Failed to save image" and the path

- [x] **Task 6: Remove old debug println! statements**
  - Remove or replace any existing `println!` debug statements with proper `log` macros
  - Keep only user-facing output if any (none expected)
  - **Acceptance Criteria:**
    - No `println!` statements remain in `main.rs` (except intentional user output)
    - `cargo clippy -- -D warnings` passes

- [x] **Task 7: Create test_images directory and sample.png**
  - Create `test_images/` directory in project root
  - Add `sample.png` - a 100x100 pixel image with a gradient pattern
  - **Acceptance Criteria:**
    - File `test_images/sample.png` exists and is a valid PNG
    - Image dimensions are 100x100 pixels

- [x] **Task 8: Create placeholder params file**
  - Create `test_images/mirror_params.json` with empty JSON object `{}`
  - **Acceptance Criteria:**
    - File `test_images/mirror_params.json` exists
    - File contains valid JSON

- [x] **Task 9: Verify full integration**
  - Run `cargo build` - must succeed
  - Run `cargo run -- -i test_images/sample.png -o output.png --plugin mirror_plugin --params test_images/mirror_params.json`
  - Run with `RUST_LOG=debug` to verify dimension logging
  - Run with non-existent input to verify error message
  - Run `cargo clippy -- -D warnings` - must pass
  - Run `cargo test` - all existing tests must pass
  - **Acceptance Criteria:**
    - All verification steps pass without errors
    - Output image `output.png` is created and identical to input (no processing yet)
    - Error messages are clear and include file paths

---

## Notes

- Plugin loading is NOT implemented in this iteration (out of scope)
- Parameter file reading is NOT implemented (file existence only)
- Only PNG format is required; other formats may work but are not tested
- No unit tests for image I/O in this iteration; integration via CLI is sufficient
