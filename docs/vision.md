# Image FFI Project - Technical Vision

## 1. Development Principles

**Core Philosophy**: KISS - Keep It Simple, Stupid

1. **Error Handling**: Use `anyhow::Result` everywhere. No custom error types.

2. **Unsafe Code Documentation**: Every `unsafe` block must have a `// SAFETY:` comment explaining:
   - What invariants must hold
   - Why those invariants are satisfied
   - What could go wrong if they weren't

3. **No Premature Abstraction**: Write straightforward code first. Refactor only when duplication becomes painful.

4. **Minimal Dependencies**: Only use crates explicitly mentioned (`clap`, `image`, `libloading`, `log`, `env_logger`, `anyhow`).

5. **No Unwrap**: Use `?` operator or `.expect("reason")` with clear message. Never bare `.unwrap()`.

## 2. Project Structure

```
image_ffi_project/
├── Cargo.toml                  # Workspace definition
├── docs/
│   ├── idea.md                 # Requirements
│   └── vision.md               # This document
├── image_processor/            # Main CLI application
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs             # Entry point, CLI args
│       └── plugin_loader.rs    # Dynamic library loading
├── mirror_plugin/
│   ├── Cargo.toml              # crate-type = ["cdylib"]
│   └── src/lib.rs              # process_image implementation
├── blur_plugin/
│   ├── Cargo.toml              # crate-type = ["cdylib"]
│   └── src/lib.rs              # process_image implementation
└── README.md
```

**Key decisions**:
- No `lib.rs` in image_processor - everything in `main.rs` and `plugin_loader.rs`
- No `error.rs` - using anyhow, no custom errors
- No shared crate - FFI signature duplicated (it's just one function)
- Each plugin is self-contained

## 3. Project Architecture

**Data Flow**:
```
┌─────────────────────────────────────────────────────────────┐
│                        main.rs                              │
│  1. Parse CLI args (clap)                                   │
│  2. Load image → RgbaImage → Vec<u8>                        │
│  3. Read params file → String                               │
│  4. Call plugin_loader::process(plugin, data, params)       │
│  5. Convert Vec<u8> back → RgbaImage → save PNG             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    plugin_loader.rs                         │
│  pub fn process(                                            │
│      plugin_path: &Path,                                    │
│      width: u32,                                            │
│      height: u32,                                           │
│      rgba_data: &mut [u8],                                  │
│      params: &str                                           │
│  ) -> Result<()>                                            │
│                                                             │
│  - Loads .so/.dll/.dylib with libloading                    │
│  - Gets process_image symbol                                │
│  - Calls it (unsafe, with SAFETY comment)                   │
│  - Library dropped after call                               │
└─────────────────────────────────────────────────────────────┘
```

**Key decisions**:
- All `unsafe` code isolated in `plugin_loader.rs`
- `main.rs` never touches raw pointers
- Plugin library is loaded, used, and dropped in one function call
- No global state

## 4. Data Model

**CLI Arguments** (clap derive):
```rust
#[derive(Parser)]
struct Args {
    /// Path to input PNG image
    input: PathBuf,
    /// Path to save output PNG image
    output: PathBuf,
    /// Plugin name (without extension)
    plugin: String,
    /// Path to JSON parameters file
    params: PathBuf,
    /// Directory containing plugins
    #[arg(default_value = "target/debug")]
    plugin_path: PathBuf,
}
```

**Image Data**: `image::RgbaImage` → `Vec<u8>` (flat RGBA buffer)

**Plugin Parameters**: JSON files, parsed by plugins themselves
```json
// mirror params example
{"horizontal": true, "vertical": false}

// blur params example
{"radius": 5, "iterations": 2}
```

**Key decisions**:
- Main app just reads params file as String, doesn't parse JSON
- Each plugin parses its own params (they know their format)
- No intermediate structs for image data - just raw bytes

## 5. Working with Plugins

**FFI Contract**:
```rust
#[no_mangle]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) -> i32  // Returns 0 on success, negative error code on failure
```

**Plugin Structure** (each plugin):
```rust
use std::ffi::CStr;
use serde::Deserialize;

#[derive(Deserialize)]
struct Params {
    // plugin-specific fields
}

#[no_mangle]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) -> i32 {
    // SAFETY: params is a valid C string from the host
    let params_str = unsafe { CStr::from_ptr(params) }.to_str().unwrap_or("");

    let params: Params = match serde_json::from_str(params_str) {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to parse params: {}", e);
            return -1; // Parse error
        }
    };

    let len = (width * height * 4) as usize;
    // SAFETY: rgba_data points to a valid buffer of len bytes, owned by the host
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, len) };

    // Process data in-place...

    0 // Success
}
```

**Safety rules**:
- Never allocate memory that the host must free
- Never read/write beyond `width * height * 4` bytes
- Handle all errors gracefully - never panic across FFI boundary

## 6. Workflows

**Build**:
```bash
# Build everything (app + all plugins)
cargo build

# Plugins appear in target/debug/ as:
# - libmirror_plugin.dylib (macOS)
# - libmirror_plugin.so (Linux)
# - mirror_plugin.dll (Windows)
```

**Usage**:
```bash
# Apply mirror plugin
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params test_images/mirror_params.json

# Apply blur plugin
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin blur_plugin \
    --params test_images/blur_params.json
```

**Testing**:
```bash
# Run all tests
cargo test

# Run specific plugin tests
cargo test -p mirror_plugin
cargo test -p blur_plugin
```

**Test structure**:
- Unit tests: In each plugin's `lib.rs` (test the image processing logic)
- Integration tests: In `image_processor/tests/` (test full CLI workflow)
- Test images: `test_images/` directory with small sample PNGs

## 7. Logging Approach

**Setup**: `env_logger` initialized once in `main.rs`:
```rust
fn main() -> Result<()> {
    env_logger::init();
    // ...
}
```

**Log levels used**:
- `info!` - Major steps (loading image, calling plugin, saving result)
- `debug!` - Details (image dimensions, params content, plugin path)
- `error!` - Failures (file not found, plugin load error, parse error)

**Example output** (with `RUST_LOG=info`):
```
[INFO] Loading image from input.png
[INFO] Image loaded: 800x600 pixels
[INFO] Loading plugin: mirror_plugin
[INFO] Calling process_image with params: {"horizontal": true}
[INFO] Saving result to output.png
[INFO] Done
```

**Plugins**: Use `log` crate macros, host's `env_logger` captures them:
```rust
// In plugin
log::debug!("Mirror: horizontal={}, vertical={}", h, v);
log::info!("Processing complete");
```

**Usage**:
```bash
# Normal run (no logs)
./image_processor ...

# With info logs
RUST_LOG=info ./image_processor ...

# With debug logs
RUST_LOG=debug ./image_processor ...
```
