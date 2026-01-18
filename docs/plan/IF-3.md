# Plan: IF-3 - Image I/O

**Status:** PLAN_APPROVED

## Overview

Implement image loading and saving functionality for the CLI application using the `image` crate. This extends the current `main.rs` to load PNG files, extract dimensions and raw RGBA data, and save the result. Additionally, initialize logging via `env_logger` and create a test image in `test_images/` directory.

---

## Components

### 1. Logging Initialization

**Location:** `image_processor/src/main.rs`

Add `env_logger::init()` at the start of `main()` to enable logging based on `RUST_LOG` environment variable.

### 2. Image Loading

**Location:** `image_processor/src/main.rs`

Use `image::open()` to load PNG files and convert to RGBA8 format:

```rust
let img = image::open(&args.input)
    .with_context(|| format!("Failed to load image: {}", args.input.display()))?
    .into_rgba8();
```

### 3. Dimension and Data Extraction

**Location:** `image_processor/src/main.rs`

Extract width, height, and raw bytes:

```rust
let (width, height) = img.dimensions();
let rgba_data: Vec<u8> = img.into_raw();
```

### 4. Image Saving

**Location:** `image_processor/src/main.rs`

Reconstruct and save the image:

```rust
let output_img = RgbaImage::from_raw(width, height, rgba_data)
    .expect("Buffer size mismatch - should never happen with unchanged data");

output_img.save(&args.output)
    .with_context(|| format!("Failed to save image: {}", args.output.display()))?;
```

### 5. Test Image

**Location:** `test_images/sample.png`

Create a simple test image (100x100 pixels) with a gradient pattern for development and testing.

---

## API Contract

### Image Data Interface

This iteration establishes the internal data format that will be passed to plugins in future iterations:

| Property | Type | Description |
|----------|------|-------------|
| width | `u32` | Image width in pixels |
| height | `u32` | Image height in pixels |
| rgba_data | `Vec<u8>` | Flat buffer, 4 bytes per pixel (R, G, B, A) |
| buffer_size | formula | Always `width * height * 4` bytes |

### Function Signatures (Internal)

No separate functions are extracted in this iteration. Following KISS principle, all logic remains inline in `main()`. Functions may be extracted in future iterations if needed for testing or clarity.

---

## Data Flows

```
CLI Arguments (from IF-2)
       |
       v
+-------------------+
|  env_logger::init |  <- Initialize logging
+-------------------+
       |
       v
+-------------------+
|   image::open()   |  <- Load PNG from args.input
+-------------------+
       |
       v
+-------------------+
|   .into_rgba8()   |  <- Convert to RGBA8 format
+-------------------+
       |
       v
+-------------------+
|  .dimensions()    |  <- Extract (width, height)
+-------------------+
       |
       v
+-------------------+
|   .into_raw()     |  <- Get Vec<u8> buffer
+-------------------+
       |
       v
+-------------------+
| [Future: Plugin]  |  <- Placeholder for plugin processing
+-------------------+
       |
       v
+-------------------+
| RgbaImage::from_  |  <- Reconstruct from buffer
|   raw()           |
+-------------------+
       |
       v
+-------------------+
|     .save()       |  <- Save PNG to args.output
+-------------------+
       |
       v
+-------------------+
|    Return Ok(())  |  <- Success
+-------------------+
```

**Error Flow:**
- File not found -> `anyhow` error with context: "Failed to load image: <path>"
- Invalid PNG -> `anyhow` error with context from image crate
- Cannot write output -> `anyhow` error with context: "Failed to save image: <path>"

---

## Non-Functional Requirements (NFRs)

| NFR | Requirement | How Addressed |
|-----|-------------|---------------|
| Error Handling | Clear, actionable error messages | Use `anyhow::Context` for all I/O operations |
| Logging | Debug-level visibility of operations | Log image dimensions and buffer size at debug level |
| Performance | Handle typical image sizes | No optimization needed; `image` crate is efficient |
| Correctness | RGBA data integrity preserved | Buffer size verified by `from_raw()` check |
| Testability | Verifiable via sample image | Test image provided in `test_images/sample.png` |

---

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Large image memory usage | Low | Medium | Out of scope for optimization; document expectations |
| PNG compression differences | Low | Low | Test compares RGBA data, not file bytes |
| Output directory does not exist | Medium | Low | Clear error message via anyhow context |
| Input file not found | Medium | Low | Clear error message via anyhow context |
| Invalid/corrupted PNG | Low | Low | Image crate error wrapped with context |

---

## Implementation Steps

1. **Add imports** to `main.rs`:
   - `use anyhow::Context;`
   - `use image::RgbaImage;`
   - `use log::{debug, info};`

2. **Initialize logging**:
   - Add `env_logger::init();` at start of `main()`

3. **Load image**:
   - Call `image::open(&args.input)` with context
   - Convert to RGBA8 with `.into_rgba8()`

4. **Extract data**:
   - Get dimensions with `.dimensions()`
   - Get raw bytes with `.into_raw()`
   - Log dimensions at debug level

5. **Save image**:
   - Reconstruct with `RgbaImage::from_raw()`
   - Save with `.save()` and context

6. **Remove debug println! statements**:
   - Replace with proper logging

7. **Create test image**:
   - Create `test_images/` directory
   - Add `sample.png` (100x100 gradient image)

8. **Verification**:
   - Run `cargo build`
   - Run `cargo run -- --input test_images/sample.png --output output.png --plugin mirror_plugin --params params.json`
   - Run with `RUST_LOG=debug` to verify logging
   - Run with non-existent input to verify error message
   - Run `cargo clippy -- -D warnings`
   - Run `cargo test`

---

## Files to Modify

| File | Change |
|------|--------|
| `image_processor/src/main.rs` | Add image loading, saving, and logging |

## Files to Create

| File | Description |
|------|-------------|
| `test_images/sample.png` | 100x100 test image with gradient pattern |
| `test_images/mirror_params.json` | Placeholder params file for testing |

---

## Out of Scope

The following are explicitly NOT part of this iteration:

- Plugin loading or calling
- Parameter file reading (file exists, but not parsed)
- Any actual image processing/transformation
- Support for formats other than PNG
- Memory optimization for large images
- Unit tests for image I/O (integration test via CLI is sufficient)

---

## Acceptance Criteria

1. `cargo build` completes without errors
2. `cargo run -- -i test_images/sample.png -o out.png ...` copies image unchanged
3. Loading non-existent file produces clear error with path
4. `RUST_LOG=debug` shows image dimensions in output
5. `cargo clippy -- -D warnings` passes
6. `cargo test` passes (existing tests still work)

---

## Open Questions

None - all requirements are clearly specified in the PRD and research documents.

---

## Alternatives Considered

### Alternative 1: Helper Functions

Extract image loading and saving into separate functions like `load_image(path) -> Result<(u32, u32, Vec<u8>)>`.

**Decision:** Deferred. Per KISS principle, keep code inline in `main()` for this iteration. Extract only if needed for testing or if complexity grows.

### Alternative 2: ImageData Struct

Create a struct to hold width, height, and data together:
```rust
struct ImageData {
    width: u32,
    height: u32,
    data: Vec<u8>,
}
```

**Decision:** Deferred. Adds abstraction without clear benefit at this stage. Will reconsider when plugin interface is implemented.

### Alternative 3: Separate image.rs Module

Create `image_processor/src/image.rs` for image operations.

**Decision:** Rejected. Conventions specify only `main.rs` and `plugin_loader.rs`. Image I/O belongs in `main.rs`.
