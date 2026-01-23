# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

CLI application for image processing with dynamically loaded plugins via FFI. Main binary loads PNG images, calls plugins to process RGBA data in-place, and saves results.

## Build & Test Commands

```bash
cargo build                    # Build all (app + plugins)
cargo test                     # Run all tests
cargo test -p image_processor  # Test main app only
cargo test -p mirror_plugin    # Test mirror plugin only
cargo test -p blur_plugin      # Test blur plugin only
cargo test test_name           # Run a single test by name
cargo fmt                      # Format code
cargo clippy -- -D warnings    # Lint with warnings as errors
```

## Run Commands

```bash
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params test_images/mirror_params.json

# With logging
RUST_LOG=debug ./target/debug/image_processor ...
```

## Architecture

Cargo workspace with 3 crates:

```
Cargo.toml                     # Workspace root
image_processor/src/
├── main.rs                    # CLI args (clap), image I/O (image crate)
└── plugin_loader.rs           # FFI loading (libloading), ALL unsafe code here

mirror_plugin/src/lib.rs       # cdylib: horizontal/vertical flip
blur_plugin/src/lib.rs         # cdylib: weighted average blur
```

**Data flow**: PNG → RgbaImage → Vec<u8> → plugin modifies in-place → save PNG

**FFI contract** (all plugins export):
```rust
#[no_mangle]
pub extern "C" fn process_image(
    width: u32, height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) -> i32  // Returns 0 on success, negative error code on failure
```

## Development Rules

See `docs/conventions.md` for complete rules. Key points:

- **KISS principle** - no premature abstraction
- **Error handling**: `anyhow::Result` only, no custom errors, no bare `.unwrap()`
- **Unsafe code**: Every `unsafe` block requires `// SAFETY:` comment
- **Dependencies**: Only clap, image, libloading, log, env_logger, anyhow, serde/serde_json
- **Plugin safety**: In-place modification only, never exceed `width * height * 4` bytes, never panic across FFI

## AI-Driven Workflow

This project uses an AI-driven feature development workflow with quality gates. Key commands:

```bash
/feature-development IF-N "Title" docs/phase/phase-N.md  # Full workflow
/validate IF-N                                            # Check gate status
/implement-orchestrated IF-N                              # Implement tasks
```

Active ticket stored in `docs/.active_ticket`. Artifacts in `docs/prd/`, `docs/plan/`, `docs/tasklist/`, `reports/qa/`.

## Reference Documentation

- `docs/idea.md` - Requirements and pre-submission checklist
- `docs/vision.md` - Technical architecture and detailed design
- `docs/conventions.md` - Code conventions and rules
- `docs/workflow.md` - AI-driven feature development workflow and slash commands
