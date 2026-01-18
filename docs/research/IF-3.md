# Research: IF-3 - Image I/O

**Status:** RESEARCH_COMPLETE

## Summary

This research document analyzes the requirements and codebase context for implementing image loading and saving using the `image` crate for the image processing application.

---

## Resolved Questions

| Question | Answer |
|----------|--------|
| Open Questions from PRD | None - PRD states "The requirements are clear from the project documentation" |
| Implementation preferences | User confirmed: "Use defaults" - proceed with documented requirements |

---

## Related Modules/Services

### Current State of `main.rs`

**File:** `image_processor/src/main.rs`

The file already contains the CLI argument parsing implementation from IF-2:

```rust
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Path to input PNG image
    #[arg(long)]
    input: PathBuf,

    /// Path to save output PNG image
    #[arg(long)]
    output: PathBuf,

    /// Plugin name (without extension)
    #[arg(long)]
    plugin: String,

    /// Path to JSON parameters file
    #[arg(long)]
    params: PathBuf,

    /// Directory containing plugins
    #[arg(long, default_value = "target/debug")]
    plugin_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Input: {:?}", args.input);
    println!("Output: {:?}", args.output);
    println!("Plugin: {}", args.plugin);
    println!("Params: {:?}", args.params);
    println!("Plugin path: {:?}", args.plugin_path);

    Ok(())
}
```

The current implementation needs to be extended to:
1. Initialize `env_logger` for logging
2. Load the PNG image from `args.input`
3. Convert to `RgbaImage` and extract dimensions
4. Get raw bytes as `Vec<u8>` (for future plugin processing)
5. Save the image to `args.output`

### Current Dependencies

**File:** `image_processor/Cargo.toml`

All required dependencies are already configured:

| Dependency | Version | Status | Purpose for IF-3 |
|------------|---------|--------|------------------|
| `clap` | 4 (with derive feature) | Ready | Already used for CLI |
| `image` | 0.25 | Ready | **Primary dependency for this ticket** |
| `libloading` | 0.9.0 | Ready | Not needed for this ticket |
| `log` | 0.4 | Ready | Logging image operations |
| `env_logger` | 0.11 | Ready | Initialize logging in main |
| `anyhow` | 1 | Ready | Error handling with context |

### Related Files

| File | Relevance |
|------|-----------|
| `image_processor/src/plugin_loader.rs` | Stub only - will receive image data in future iteration |
| `mirror_plugin/src/lib.rs` | No-op stub - not affected by this ticket |
| `blur_plugin/src/lib.rs` | No-op stub - not affected by this ticket |
| `test_images/` | **Directory does not exist** - must be created with sample image |

---

## Current Endpoints and Contracts

### Data Flow (from vision.md)

```
1. Parse CLI args (clap)              [DONE - IF-2]
2. Load image -> RgbaImage -> Vec<u8>  [THIS ITERATION]
3. Read params file -> String          [FUTURE]
4. Call plugin_loader::process(...)    [FUTURE]
5. Convert Vec<u8> back -> RgbaImage -> save PNG  [THIS ITERATION]
```

### Image Data Contract

Based on `docs/vision.md` and `docs/idea.md`:

| Property | Type | Description |
|----------|------|-------------|
| width | `u32` | Image width in pixels |
| height | `u32` | Image height in pixels |
| rgba_data | `Vec<u8>` | Flat buffer, 4 bytes per pixel (R, G, B, A) |
| buffer_size | formula | Always `width * height * 4` bytes |

### Expected Image Crate Types

| Type | Description |
|------|-------------|
| `image::DynamicImage` | Loaded image (any format) |
| `image::RgbaImage` | Image in RGBA8 format (`ImageBuffer<Rgba<u8>, Vec<u8>>`) |
| `image::Rgba<u8>` | Single pixel type (4 bytes) |

---

## Patterns Used

### Image Loading Pattern

From `docs/vision.md` and `docs/idea.md`:

```rust
use image::{open, RgbaImage};

// Load and convert to RGBA
let img: RgbaImage = open(&args.input)
    .with_context(|| format!("Failed to load image: {}", args.input.display()))?
    .into_rgba8();

// Extract dimensions
let (width, height) = img.dimensions();

// Get raw bytes
let rgba_data: Vec<u8> = img.into_raw();
```

### Image Saving Pattern

From `docs/vision.md`:

```rust
use image::{RgbaImage, save_buffer, ColorType};

// Reconstruct image from raw bytes
let output_img = RgbaImage::from_raw(width, height, rgba_data)
    .expect("Buffer size mismatch - this should never happen");

// Save as PNG
output_img.save(&args.output)
    .with_context(|| format!("Failed to save image: {}", args.output.display()))?;
```

### Error Handling Pattern

From `docs/conventions.md`:

- Use `anyhow::Result` with `.with_context()` for descriptive errors
- No bare `.unwrap()` - use `?` operator or `.expect("reason")`
- File operations must return `Result` with meaningful error messages

Example:
```rust
use anyhow::{Context, Result};

fn load_image(path: &Path) -> Result<(u32, u32, Vec<u8>)> {
    let img = image::open(path)
        .with_context(|| format!("Failed to load image: {}", path.display()))?
        .into_rgba8();

    let (width, height) = img.dimensions();
    let data = img.into_raw();

    Ok((width, height, data))
}
```

### Logging Pattern

From `docs/vision.md`:

```rust
use log::{info, debug, error};

fn main() -> Result<()> {
    env_logger::init();

    info!("Loading image from {}", args.input.display());
    debug!("Image loaded: {}x{} pixels", width, height);
    info!("Saving result to {}", args.output.display());
}
```

Logging should be enabled via environment variable: `RUST_LOG=info` or `RUST_LOG=debug`

---

## Limitations and Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Large image memory usage | Low | Medium | Document expectations; out of scope for optimization |
| PNG format variations | Low | Low | `image` crate handles conversion to RGBA8 automatically |
| Output directory not existing | Medium | Low | Error will be clear from `anyhow` context |
| File permission errors | Low | Low | Error handling via anyhow provides clear messages |
| Input file not found | Medium | Low | Error will be clear from `anyhow` context |
| Invalid/corrupted PNG | Low | Low | `image` crate returns error; wrap with context |

### Scope Boundaries

This ticket INCLUDES:
- PNG loading via `image` crate
- Conversion to RGBA8 format
- Dimension extraction (width, height)
- Raw byte access as `Vec<u8>`
- PNG saving via `image` crate
- Adding a test image to `test_images/`
- Logging initialization and usage
- Error handling with context

This ticket does NOT include:
- Plugin loading or calling
- Parameter file reading
- Any actual image processing
- Support for formats other than PNG (though `image` crate may support them)

---

## New Technical Questions

### For Follow-up (Not Blocking)

1. **Test image source:** A sample PNG image needs to be added to `test_images/`. Options:
   - Create a small solid-color PNG programmatically
   - Use a simple test pattern image
   - **Recommendation:** Create a small (e.g., 100x100) PNG with a simple pattern

2. **Byte-for-byte identity:** The acceptance criteria states output should be "byte-for-byte identical" when no processing is applied. However, PNG encoding may differ slightly due to compression settings.
   - **Clarification:** The RGBA data should be identical; the PNG file bytes may differ.
   - **Test approach:** Load both input and output, compare RGBA buffers.

3. **Function extraction:** The PRD does not specify whether to create helper functions or keep everything in `main()`.
   - **Recommendation per KISS:** Start with inline code in `main()`, extract functions only if needed for clarity or testing.

---

## Implementation Checklist

For IF-3 implementation, ensure:

- [ ] Add `use image::{open, RgbaImage};` import
- [ ] Add `use anyhow::Context;` for `.with_context()`
- [ ] Add `use log::{info, debug};` for logging
- [ ] Initialize `env_logger::init();` at start of main
- [ ] Load image: `image::open(&args.input)?.into_rgba8()`
- [ ] Extract dimensions: `img.dimensions()`
- [ ] Get raw bytes: `img.into_raw()`
- [ ] Log dimensions at debug level
- [ ] Reconstruct image: `RgbaImage::from_raw(width, height, data)`
- [ ] Save image: `output_img.save(&args.output)?`
- [ ] Add error context to all fallible operations
- [ ] Create `test_images/` directory
- [ ] Add sample test PNG image (e.g., `sample.png`)
- [ ] Verify: `cargo run -- --input test_images/sample.png --output output.png --plugin mirror_plugin --params params.json` copies image
- [ ] Verify: Invalid input path produces clear error
- [ ] Verify: `RUST_LOG=debug` shows dimensions
- [ ] Run `cargo clippy -- -D warnings`
- [ ] Run `cargo test`

---

## Code Snippets for Reference

### Minimal Implementation

```rust
use anyhow::{Context, Result};
use clap::Parser;
use image::RgbaImage;
use log::{debug, info};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Path to input PNG image
    #[arg(long)]
    input: PathBuf,

    /// Path to save output PNG image
    #[arg(long)]
    output: PathBuf,

    /// Plugin name (without extension)
    #[arg(long)]
    plugin: String,

    /// Path to JSON parameters file
    #[arg(long)]
    params: PathBuf,

    /// Directory containing plugins
    #[arg(long, default_value = "target/debug")]
    plugin_path: PathBuf,
}

fn main() -> Result<()> {
    env_logger::init();
    let args = Args::parse();

    // Load image
    info!("Loading image from {}", args.input.display());
    let img = image::open(&args.input)
        .with_context(|| format!("Failed to load image: {}", args.input.display()))?
        .into_rgba8();

    // Extract dimensions and raw bytes
    let (width, height) = img.dimensions();
    debug!("Image loaded: {}x{} pixels", width, height);

    let rgba_data = img.into_raw();
    debug!("Buffer size: {} bytes", rgba_data.len());

    // TODO: Plugin processing will go here in future iteration
    // For now, pass through unchanged

    // Save image
    info!("Saving result to {}", args.output.display());
    let output_img = RgbaImage::from_raw(width, height, rgba_data)
        .expect("Buffer size mismatch - this should never happen with unchanged data");

    output_img.save(&args.output)
        .with_context(|| format!("Failed to save image: {}", args.output.display()))?;

    info!("Done");
    Ok(())
}
```

### Test Image Creation (for test_images/sample.png)

A simple test image can be created using the `image` crate in a test or build script:

```rust
use image::{RgbaImage, Rgba};

fn create_test_image() {
    let width = 100;
    let height = 100;
    let mut img = RgbaImage::new(width, height);

    // Create a simple gradient pattern
    for y in 0..height {
        for x in 0..width {
            img.put_pixel(x, y, Rgba([
                (x * 255 / width) as u8,   // R: gradient left-to-right
                (y * 255 / height) as u8,  // G: gradient top-to-bottom
                128,                        // B: constant
                255,                        // A: fully opaque
            ]));
        }
    }

    img.save("test_images/sample.png").expect("Failed to save test image");
}
```

---

## References

- `docs/idea.md` - Full project requirements (lines 35-43 for image I/O)
- `docs/vision.md` - Data flow and image handling patterns (lines 50-60)
- `docs/conventions.md` - Error handling and KISS principle
- `docs/prd/IF-3.prd.md` - PRD for this ticket
- `docs/phase/phase-3.md` - Phase definition and acceptance criteria
- `image_processor/src/main.rs` - Current CLI implementation to extend
- `image_processor/Cargo.toml` - Dependencies (image 0.25 already configured)
- [image crate documentation](https://docs.rs/image/) - API reference
