# IF-4: Plugin Loader - Tasklist

**Status:** IMPLEMENT_STEP_OK

## Context

Implement the plugin loader module to dynamically load plugin libraries via FFI and call the `process_image` function. This isolates all unsafe code in a single module and provides a safe Rust interface for the rest of the application.

## Tasks

### Module Setup

- [x] **T1: Create plugin_loader.rs module structure**
  - Add necessary imports: `std::ffi::{c_char, CString}`, `std::path::Path`, `libloading::Library`, `anyhow::{Result, Context}`, `log::{debug, info}`
  - Define the `ProcessImageFn` type alias for the FFI function signature
  - **Acceptance:** Module compiles with imports and type alias defined

### Core Implementation

- [x] **T2: Implement library_filename() function**
  - Use `cfg!(target_os = ...)` for platform detection
  - Return `lib{name}.dylib` for macOS, `lib{name}.so` for Linux, `{name}.dll` for Windows
  - **Acceptance:** Function returns correct filename for each platform

- [x] **T3: Implement process() function - library loading**
  - Load library using `Library::new()` with `unsafe` block
  - Add proper SAFETY comment explaining invariants
  - Use `anyhow::Context` for error message with expected path
  - **Acceptance:** Function loads library and returns descriptive error if missing

- [x] **T4: Implement process() function - symbol resolution**
  - Get `process_image` symbol using `lib.get::<ProcessImageFn>()`
  - Add proper SAFETY comment explaining symbol requirements
  - Use `anyhow::Context` for error message if symbol missing
  - **Acceptance:** Function resolves symbol and returns error if not found

- [x] **T5: Implement process() function - FFI call**
  - Convert params to `CString` using `CString::new()`
  - Call `process_image` with `unsafe` block
  - Add proper SAFETY comment explaining pointer validity
  - **Acceptance:** Function calls plugin with correct arguments

- [x] **T6: Add logging to process() function**
  - Add `debug!` logs for library path, dimensions, and params
  - Add `info!` logs for plugin load start and completion
  - **Acceptance:** Logs appear when running with `RUST_LOG=debug`

### Integration

- [x] **T7: Integrate plugin_loader into main.rs**
  - Add `mod plugin_loader;` declaration
  - Read params file using `std::fs::read_to_string()`
  - Build plugin library path using `library_filename()`
  - Call `plugin_loader::process()` after loading image, before saving
  - **Acceptance:** main.rs compiles and calls plugin loader

### Testing

- [x] **T8: Add unit test for library_filename()**
  - Test returns correct suffix for current platform
  - **Acceptance:** Test passes with `cargo test -p image_processor`

- [x] **T9: Add integration test for plugin loading**
  - Create test that loads a real plugin (mirror_plugin stub)
  - Verify process() succeeds with valid plugin
  - Verify process() returns error with invalid plugin path
  - **Acceptance:** Test passes with `cargo test -p image_processor`

### Verification

- [x] **T10: Verify all SAFETY comments are present**
  - Every `unsafe` block has a `// SAFETY:` comment
  - Comments explain what invariants must hold and why they are satisfied
  - **Acceptance:** Code review confirms SAFETY comments on all unsafe blocks

- [x] **T11: Run full build and lint checks**
  - Run `cargo build` - no errors
  - Run `cargo clippy -- -D warnings` - no warnings
  - Run `cargo fmt --check` - no formatting issues
  - **Acceptance:** All commands pass without errors or warnings
