# IF-7: Final Polish - Research Document

## Resolved Questions

**User Preference:** "Use defaults" - Proceed with documented requirements only.

No additional constraints or implementation details were provided. The implementation will follow the PRD and project conventions exactly as documented.

---

## Related Modules and Services

### Project Structure

The project is a Cargo workspace with three crates:

```
image_ffi_project/
├── Cargo.toml                           # Workspace definition (resolver = "3")
├── image_processor/                     # Main CLI application
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs                      # CLI args (clap), image I/O, env_logger::init()
│       └── plugin_loader.rs             # FFI loading (libloading), all unsafe code
├── mirror_plugin/                       # cdylib: horizontal/vertical flip
│   ├── Cargo.toml
│   └── src/lib.rs
├── blur_plugin/                         # cdylib: weighted average blur
│   ├── Cargo.toml
│   └── src/lib.rs
└── test_images/                         # Test resources
    ├── sample.png
    ├── mirror_params.json
    └── blur_params.json
```

### Key Files for IF-7

| File | Relevance |
|------|-----------|
| `image_processor/src/main.rs` | Verify logging, error handling, add integration tests |
| `image_processor/src/plugin_loader.rs` | Verify error handling, logging |
| `mirror_plugin/src/lib.rs` | Verify unit test coverage |
| `blur_plugin/src/lib.rs` | Verify unit test coverage |
| `test_images/` | Resources for integration tests |
| (missing) `README.md` | Needs to be created |
| (missing) `image_processor/tests/` | Integration test directory needs to be created |

---

## Current Endpoints and Contracts

### CLI Interface (main.rs)

```rust
struct Args {
    #[arg(long)]
    input: PathBuf,         // Path to input PNG image

    #[arg(long)]
    output: PathBuf,        // Path to save output PNG image

    #[arg(long)]
    plugin: String,         // Plugin name (without extension)

    #[arg(long)]
    params: PathBuf,        // Path to JSON parameters file

    #[arg(long, default_value = "target/debug")]
    plugin_path: PathBuf,   // Directory containing plugins
}
```

### Plugin FFI Contract

All plugins export:

```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
)
```

### Plugin Parameters (JSON)

**mirror_plugin:**
```json
{"horizontal": true, "vertical": false}
```

**blur_plugin:**
```json
{"radius": 3, "iterations": 1}
```

---

## Patterns Used

### Existing Logging Pattern

**Location:** `image_processor/src/main.rs` and `image_processor/src/plugin_loader.rs`

```rust
// Initialization in main.rs line 33
env_logger::init();

// Usage patterns found:
log::debug!("...");   // Details (image dimensions, params content, plugin path)
log::info!("...");    // Major steps (loading plugin, saving result)
log::error!("...");   // Failures (used in plugins for JSON parse errors)
```

**Current log statements:**
- `main.rs:45`: `debug!("Loaded image: {}x{} ({} bytes)", ...)`
- `main.rs:72`: `info!("Saved image to: {}", ...)`
- `plugin_loader.rs:39`: `debug!("plugin_loader::process called with path=..., dimensions=..., params=...")`
- `plugin_loader.rs:47`: `info!("Loading plugin from: {}", ...)`
- `plugin_loader.rs:83`: `info!("Plugin execution complete")`
- `mirror_plugin:36`: `error!("mirror_plugin: failed to parse params JSON: {}", ...)`
- `blur_plugin:43`: `error!("blur_plugin: failed to parse params JSON: {}", ...)`

### Existing Error Handling Pattern

**Pattern:** Use `anyhow::Result` with `.with_context()` for meaningful messages.

```rust
// Example from main.rs
let img = image::open(&args.input)
    .with_context(|| format!("Failed to load image: {}", args.input.display()))?
    .into_rgba8();

// Example from plugin_loader.rs
let lib = unsafe { Library::new(plugin_path) }
    .with_context(|| format!("Failed to load plugin library: {}", plugin_path.display()))?;
```

**Single `.expect()` usage:**
- `main.rs:65`: `RgbaImage::from_raw(...).expect("Buffer size mismatch - plugin must not change buffer size")` - This is acceptable per conventions as it indicates a plugin programming error.

### Existing Test Pattern

**Unit tests in plugins:**
- Helper functions to call `process_image` with test data
- Test image creation functions (`create_4x4_test_image()`, `create_3x3_test_image()`)
- Pixel getter functions for verification
- Tests cover: normal operation, edge cases (1x1 image, odd dimensions), invalid JSON, empty JSON, partial params

**Unit tests in main app:**
- CLI argument parsing tests using `Args::try_parse_from()`
- Tests cover: all arguments, default values, missing required args, path structures

**Ignored integration-style tests in plugin_loader.rs:**
- `test_process_invalid_params_with_null_byte` (requires real plugin)
- `test_process_with_real_plugin` (requires built plugin)

---

## Current Test Structure

### Test Count by Module

| Module | Test Count | Coverage |
|--------|------------|----------|
| `main.rs` | 8 tests | CLI argument parsing |
| `plugin_loader.rs` | 4 tests (2 ignored) | Library filename, missing library error, real plugin tests (ignored) |
| `mirror_plugin/lib.rs` | 14 tests | All flip scenarios, edge cases, invalid input |
| `blur_plugin/lib.rs` | 13 tests | Params parsing, blur effects, edge cases, invalid input |
| **Total** | 39 tests (37 active) | |

### Missing Test Coverage

1. **Integration tests** - No `image_processor/tests/` directory exists
2. **End-to-end workflow tests** - Load image, call plugin, verify output

---

## Integration Test Requirements

Based on the PRD and project requirements, integration tests should cover:

### Test Scenarios

1. **Full workflow with mirror_plugin**
   - Load `test_images/sample.png`
   - Apply mirror plugin with horizontal flip
   - Verify output file exists and has correct dimensions
   - Clean up test artifacts

2. **Full workflow with blur_plugin**
   - Load `test_images/sample.png`
   - Apply blur plugin with radius=1
   - Verify output file exists and has correct dimensions
   - Clean up test artifacts

3. **Error handling: non-existent input file**
   - Verify meaningful error message

4. **Error handling: non-existent plugin**
   - Verify meaningful error message

5. **Error handling: invalid params file**
   - Verify meaningful error message

### Integration Test Implementation Pattern

Based on `docs/vision.md`:

```rust
// In image_processor/tests/integration_test.rs
use std::process::Command;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_mirror_plugin_workflow() {
    // Build first
    let build = Command::new("cargo").args(["build"]).output().unwrap();
    assert!(build.status.success());

    let temp_dir = tempdir().unwrap();
    let output_path = temp_dir.path().join("output.png");

    let result = Command::new("./target/debug/image_processor")
        .args([
            "--input", "test_images/sample.png",
            "--output", output_path.to_str().unwrap(),
            "--plugin", "mirror_plugin",
            "--params", "test_images/mirror_params.json",
        ])
        .output()
        .unwrap();

    assert!(result.status.success());
    assert!(output_path.exists());

    // Verify output image dimensions match input
    let input_img = image::open("test_images/sample.png").unwrap();
    let output_img = image::open(&output_path).unwrap();
    assert_eq!(input_img.dimensions(), output_img.dimensions());
}
```

### Dependencies for Integration Tests

Need to add `tempfile` as a dev-dependency:

```toml
[dev-dependencies]
tempfile = "3"
```

---

## README Requirements

Based on `docs/idea.md` checklist and `docs/vision.md`:

### Required Sections

1. **Project Description**
   - What the project does
   - Plugin architecture overview

2. **Build Instructions**
   ```bash
   cargo build                    # Build all (app + plugins)
   ```

3. **Usage Examples**
   - Mirror plugin example
   - Blur plugin example
   - With logging enabled

4. **Test Execution**
   ```bash
   cargo test                     # Run all tests
   cargo test -p image_processor  # Test main app only
   cargo test -p mirror_plugin    # Test mirror plugin only
   cargo test -p blur_plugin      # Test blur plugin only
   ```

5. **Plugin Parameters**
   - Mirror plugin JSON format
   - Blur plugin JSON format

6. **Project Structure**
   - Overview of workspace layout

---

## Logging Verification Status

### Current Implementation (COMPLETE)

| Requirement | Status | Evidence |
|-------------|--------|----------|
| `env_logger::init()` called | PASS | `main.rs:33` |
| `log::debug!` for details | PASS | `main.rs:45`, `plugin_loader.rs:39` |
| `log::info!` for major steps | PASS | `main.rs:72`, `plugin_loader.rs:47,83` |
| `log::error!` for failures | PASS | `mirror_plugin:36`, `blur_plugin:43` |
| `RUST_LOG=info` shows logs | PASS (to verify) | Implementation present |

### Gaps Identified

The logging implementation is functionally complete. However, the PRD Scenario 1 expects these log messages:
- Loading image (missing explicit log)
- Image dimensions (present as debug)
- Loading plugin (present as info)
- Plugin execution complete (present as info)
- Saving result (present as info)

**Recommendation:** Consider adding `info!("Loading image from: {}", ...)` before `image::open()` for completeness.

---

## Error Handling Audit

### Source Code Audit Results

**Grep for `.unwrap()` in `*.rs` files:** No matches found in source code (only in test code and documentation).

**`.expect()` usage audit:**

| Location | Usage | Acceptable |
|----------|-------|------------|
| `main.rs:65` | `RgbaImage::from_raw(...).expect("Buffer size mismatch - plugin must not change buffer size")` | YES - plugin programming error |
| `main.rs:96,118,196,216` | Test code `.expect("should parse...")` | YES - test assertions |
| `mirror_plugin:90` | Test code `.expect("CString creation failed")` | YES - test helper |
| `blur_plugin:123,131,139,147,153` | Test code `.expect(...)` | YES - test assertions |

### Error Handling Completeness

| Operation | Error Handling | Evidence |
|-----------|----------------|----------|
| Image loading | `.with_context()` | `main.rs:38-40` |
| Params file reading | `.with_context()` | `main.rs:53-54` |
| Plugin library loading | `.with_context()` | `plugin_loader.rs:63-64` |
| Symbol lookup | `.with_context()` | `plugin_loader.rs:69-71` |
| CString conversion | `.with_context()` | `plugin_loader.rs:73` |
| Image saving | `.with_context()` | `main.rs:68-70` |
| Plugin JSON parsing | `match` + early return | Both plugins |

**Result:** Error handling is complete. All operations use `anyhow::Result` with meaningful context.

---

## Dependencies and Limitations

### Current Dependencies

**image_processor/Cargo.toml:**
- `clap = { version = "4", features = ["derive"] }`
- `image = "0.25"`
- `libloading = "0.9.0"`
- `log = "0.4"`
- `env_logger = "0.11"`
- `anyhow = "1"`

**Plugins:**
- `log = "0.4"`
- `serde = { version = "1", features = ["derive"] }`
- `serde_json = "1"`

### Additional Dependencies Needed

For integration tests:
- `tempfile = "3"` (dev-dependency) - for temporary output files
- `image = "0.25"` (dev-dependency) - for verifying output dimensions

### Limitations

1. **Integration tests require built plugins** - Tests must run `cargo build` first or be run via `cargo test` which builds automatically
2. **Platform-specific library names** - Tests must handle `.dylib` (macOS), `.so` (Linux), `.dll` (Windows)
3. **File system operations** - Integration tests create/delete files, could be flaky on some systems

---

## New Technical Questions

1. **Should integration tests be `#[ignore]` by default?** - They require built plugins and file I/O. Running them in CI may need `cargo build` first.

2. **Should README include troubleshooting section?** - Common issues like "plugin not found" could be documented.

3. **Should we add a `warn!` level log for edge cases?** - Currently only `debug`, `info`, and `error` are used. Edge cases like "no flip requested" silently succeed.

---

## Summary of Required Work

### Task 7.1 - Logging (Status: MOSTLY COMPLETE)
- [x] `env_logger::init()` present
- [x] Log macros used throughout
- [ ] Consider adding "Loading image" info log for completeness

### Task 7.2 - Error Handling (Status: COMPLETE)
- [x] All error paths use `anyhow::Result`
- [x] All operations have `.with_context()`
- [x] No bare `.unwrap()` in source code
- [x] Single `.expect()` is documented and acceptable

### Task 7.3 - Unit Tests (Status: COMPLETE)
- [x] 8 tests for CLI parsing
- [x] 4 tests for plugin loader (2 ignored)
- [x] 14 tests for mirror plugin
- [x] 13 tests for blur plugin

### Task 7.4 - Integration Tests (Status: NOT STARTED)
- [ ] Create `image_processor/tests/` directory
- [ ] Add integration test for mirror plugin workflow
- [ ] Add integration test for blur plugin workflow
- [ ] Add error case tests

### Task 7.5 - README.md (Status: NOT STARTED)
- [ ] Create comprehensive README with build/run/test instructions
