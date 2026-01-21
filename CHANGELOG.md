# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **IF-7: Final Polish** - Added integration tests and README documentation
  - Integration test suite in `image_processor/tests/integration_test.rs` with 6 end-to-end tests
  - Positive workflow tests for mirror and blur plugins verifying exit status, file creation, and dimension preservation
  - Error handling tests for missing input file, missing plugin, and missing params file
  - Comprehensive `README.md` with project overview, build instructions, usage examples, and project structure
  - Added info-level log message for image loading: "Loading image from: <path>"
  - Dev-dependencies `tempfile` and `image` for integration test support

- **IF-6: Blur Plugin** - Implemented weighted average blur algorithm with configurable radius and iterations parameters
  - `Params` struct with `radius` and `iterations` u32 fields using serde defaults (default: 1 each)
  - Weighted average blur using inverse distance formula: `1.0 / (distance + 1.0)`
  - Temporary buffer to avoid reading modified pixels during processing
  - Multiple iterations support for progressively stronger blur effects
  - Edge pixel handling: only averages valid neighbors within image bounds
  - Graceful error handling for invalid JSON (logs error, returns early)
  - Early return optimization when radius=0 or iterations=0
  - 12 unit tests covering parameter parsing, blur algorithm, edge cases, and error handling
  - Test parameters file `test_images/blur_params.json` with `{"radius": 3, "iterations": 1}`

- **IF-5: Mirror Plugin** - Implemented horizontal and vertical image flipping
  - `Params` struct with `horizontal` and `vertical` boolean fields using serde defaults
  - Horizontal flip: swaps pixels within each row from left to right
  - Vertical flip: swaps rows from top to bottom
  - Combined flip produces 180-degree rotation effect
  - Graceful error handling for invalid JSON (logs error, returns early)
  - In-place buffer modification with O(n) time and O(1) space complexity
  - 11 unit tests covering normal operations, edge cases (1x1, odd dimensions), and error handling
  - Updated `test_images/mirror_params.json` with default parameters

- **IF-4: Plugin Loader** - Implemented dynamic plugin loading via FFI using `libloading`
  - Created `plugin_loader.rs` module isolating all unsafe FFI code
  - Platform-specific library filename construction (`.dylib`/`.so`/`.dll`)
  - `process()` function providing safe Rust interface for plugin invocation
  - Symbol resolution for `process_image` function with proper SAFETY documentation
  - CString parameter marshalling for FFI calls
  - Integration with main.rs: params file reading and plugin invocation before save
  - Debug and info level logging for plugin loading and execution
  - Three tests covering library filename, missing library error, and real plugin integration

- **IF-3: Image I/O** - Implemented image loading and saving using the `image` crate
  - PNG loading via `image::open()` with automatic RGBA8 conversion
  - Dimension extraction via `.dimensions()` method
  - Raw RGBA bytes via `.into_raw()` as `Vec<u8>` (4 bytes per pixel)
  - Image reconstruction and saving via `RgbaImage::from_raw()` and `.save()`
  - Logging integration via `env_logger` with debug and info levels
  - Error handling with `anyhow::Context` for clear error messages including file paths
  - Test assets: `test_images/sample.png` (100x100 gradient) and `test_images/mirror_params.json`

- **IF-2: CLI Arguments** - Implemented command-line argument parsing using clap derive macros
  - `Args` struct with 5 fields: `input`, `output`, `plugin`, `params`, `plugin_path`
  - Long flags for all arguments (`--input`, `--output`, `--plugin`, `--params`, `--plugin-path`)
  - Default value `target/debug` for `--plugin-path` argument
  - Auto-generated help documentation via `--help` flag
  - 8 unit tests covering argument parsing, default values, and error cases

- **IF-1: Project Scaffolding** - Established Cargo workspace with three crates
  - Workspace `Cargo.toml` with resolver version 2
  - `image_processor` binary crate with dependencies: clap, image, libloading, log, env_logger, anyhow
  - `mirror_plugin` cdylib crate with dependencies: serde, serde_json, log
  - `blur_plugin` cdylib crate with dependencies: serde, serde_json, log
  - FFI stub functions (`process_image`) in both plugins with correct signature
  - Minimal main.rs entry point and plugin_loader.rs stub file
