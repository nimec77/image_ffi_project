# Image Processing CLI with Plugin Architecture

A command-line tool for image processing with dynamically loaded plugins via FFI (Foreign Function Interface).

## Overview

This application loads PNG images, applies transformations through dynamically loaded plugins, and saves the results. Plugins are compiled as shared libraries (cdylib) and loaded at runtime, allowing for extensible image processing capabilities without recompiling the main application.

## Architecture

```
image_ffi_project/
├── image_processor/       # Main CLI application
│   └── src/
│       ├── main.rs        # CLI args, image I/O
│       └── plugin_loader.rs  # Dynamic library loading (FFI)
├── mirror_plugin/         # Plugin: horizontal/vertical flip
├── blur_plugin/           # Plugin: weighted average blur
└── test_images/           # Sample images and parameter files
```

**Data Flow:**
1. Load PNG image into RGBA buffer
2. Load plugin shared library at runtime
3. Plugin processes RGBA data in-place
4. Save modified buffer as PNG

**FFI Contract:**

All plugins export a `process_image` function with the following signature:

```rust
#[no_mangle]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
)
```

## Prerequisites

- Rust toolchain (edition 2024)
- Cargo package manager

Install Rust via [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Building

Build the main application and all plugins:

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

Plugins are compiled to shared libraries:
- macOS: `libmirror_plugin.dylib`, `libblur_plugin.dylib`
- Linux: `libmirror_plugin.so`, `libblur_plugin.so`
- Windows: `mirror_plugin.dll`, `blur_plugin.dll`

## Testing

Run all tests:

```bash
cargo test
```

Run tests for a specific crate:

```bash
cargo test -p image_processor  # Test main app only
cargo test -p mirror_plugin    # Test mirror plugin
cargo test -p blur_plugin      # Test blur plugin
```

Run a single test by name:

```bash
cargo test test_name
```

## Usage

### CLI Syntax

```bash
./target/debug/image_processor \
    --input <INPUT_PATH> \
    --output <OUTPUT_PATH> \
    --plugin <PLUGIN_NAME> \
    --params <PARAMS_PATH> \
    [--plugin-path <PLUGIN_DIR>]
```

| Argument | Description |
|----------|-------------|
| `--input` | Path to input PNG image |
| `--output` | Path for output PNG image |
| `--plugin` | Plugin name (without lib prefix or extension) |
| `--params` | Path to JSON parameters file |
| `--plugin-path` | Optional: directory containing plugin libraries (default: `./target/debug`) |

### Mirror Plugin Example

Flip an image horizontally:

```bash
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params test_images/mirror_params.json
```

Parameter format (`mirror_params.json`):

```json
{
    "horizontal": true,
    "vertical": false
}
```

### Blur Plugin Example

Apply a blur effect:

```bash
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin blur_plugin \
    --params test_images/blur_params.json
```

Parameter format (`blur_params.json`):

```json
{
    "radius": 5,
    "iterations": 2
}
```

### Logging

Enable logging with the `RUST_LOG` environment variable:

```bash
# Info level - shows main workflow steps
RUST_LOG=info ./target/debug/image_processor ...

# Debug level - shows detailed information
RUST_LOG=debug ./target/debug/image_processor ...
```

## Project Structure

```
image_ffi_project/
├── Cargo.toml                 # Workspace root configuration
├── image_processor/           # Main CLI application
│   ├── Cargo.toml             # Dependencies: clap, image, libloading, anyhow, log
│   ├── src/
│   │   ├── main.rs            # CLI argument parsing, image I/O
│   │   └── plugin_loader.rs   # FFI plugin loading (all unsafe code here)
│   └── tests/
│       └── integration_test.rs # End-to-end CLI tests
├── mirror_plugin/             # Mirror/flip plugin (cdylib)
│   ├── Cargo.toml             # Dependencies: log, serde, serde_json
│   └── src/lib.rs             # Horizontal/vertical flip implementation
├── blur_plugin/               # Blur plugin (cdylib)
│   ├── Cargo.toml             # Dependencies: log, serde, serde_json
│   └── src/lib.rs             # Weighted average blur implementation
├── test_images/               # Test resources
│   ├── sample.png             # Sample input image
│   ├── mirror_params.json     # Mirror plugin parameters
│   └── blur_params.json       # Blur plugin parameters
└── docs/                      # Project documentation
    ├── idea.md                # Requirements and checklist
    ├── vision.md              # Technical architecture
    └── conventions.md         # Code conventions
```

### Crate Responsibilities

| Crate | Type | Purpose |
|-------|------|---------|
| `image_processor` | Binary | CLI interface, image loading/saving, plugin orchestration |
| `mirror_plugin` | cdylib | Image flip transformations (horizontal, vertical) |
| `blur_plugin` | cdylib | Weighted average blur with configurable radius and iterations |
