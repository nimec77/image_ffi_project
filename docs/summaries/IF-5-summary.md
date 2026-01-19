# IF-5 Summary: Mirror Plugin

**Status:** COMPLETED
**Date:** 2026-01-19

---

## Overview

IF-5 implemented the mirror plugin that flips images horizontally and/or vertically based on JSON parameters. The plugin processes RGBA image data via FFI, modifying the buffer in-place to achieve left-right mirroring, top-bottom flipping, or both combined for a 180-degree rotation effect. This is the first fully functional image transformation plugin in the project.

---

## What Was Implemented

### 1. Params Struct

Defined a simple configuration struct for JSON deserialization:

```rust
#[derive(Deserialize)]
struct Params {
    #[serde(default)]
    horizontal: bool,
    #[serde(default)]
    vertical: bool,
}
```

Using `#[serde(default)]` ensures missing fields default to `false`, allowing empty JSON `{}` or partial parameters `{"horizontal": true}` to work correctly.

### 2. JSON Parameter Parsing

The `process_image` function parses the C string params:

- Converts C string to Rust `&str` using `CStr::from_ptr` with `unwrap_or("")` fallback
- Deserializes JSON using `serde_json::from_str`
- On parse failure, logs error and returns early without modifying the image
- No panics across FFI boundary

### 3. Horizontal Flip Algorithm

Swaps pixels within each row from left edge toward center:

```rust
for y in 0..height {
    for x in 0..width / 2 {
        // swap pixel at (x, y) with pixel at (width-1-x, y)
    }
}
```

Uses the formula `(y * width + x) * 4` to calculate pixel indices, swapping all 4 RGBA bytes.

### 4. Vertical Flip Algorithm

Swaps entire rows from top and bottom toward center:

```rust
for y in 0..height / 2 {
    // swap row y with row (height-1-y)
}
```

Each row contains `width * 4` bytes, swapped byte-by-byte.

### 5. Combined Operations

When both `horizontal` and `vertical` are true, both flips are applied sequentially, producing a 180-degree rotation effect.

### 6. Test Parameters File

Updated `test_images/mirror_params.json` with default test parameters:

```json
{"horizontal": true, "vertical": false}
```

### 7. Comprehensive Unit Tests

Added 11 unit tests in `mirror_plugin/src/lib.rs`:

| Test | Purpose |
|------|---------|
| `test_horizontal_flip` | Verifies pixel positions after horizontal flip |
| `test_vertical_flip` | Verifies row positions after vertical flip |
| `test_combined_flip` | Verifies 180-degree rotation with both flips |
| `test_no_flip` | Confirms no-op when both flags are false |
| `test_1x1_image` | Edge case: single pixel |
| `test_odd_dimensions_horizontal` | Edge case: 3x3 horizontal flip |
| `test_odd_dimensions_vertical` | Edge case: 3x3 vertical flip |
| `test_invalid_json` | Error handling: malformed JSON |
| `test_empty_json` | Default behavior with `{}` |
| `test_partial_params` | Partial params with only one field |

Helper functions include `call_process_image()`, `create_4x4_test_image()`, `create_3x3_test_image()`, and `get_pixel()`.

---

## Key Decisions

### 1. In-Place Buffer Modification

The plugin modifies the RGBA buffer in-place without allocating temporary buffers. This follows the FFI contract and minimizes memory overhead. Swaps use Rust's standard `slice.swap()` method.

### 2. Early Return Optimization

When both `horizontal` and `vertical` are `false`, the function returns immediately without converting the raw pointer to a slice, avoiding unnecessary work.

### 3. Integer Division for Loop Bounds

Using `width / 2` and `height / 2` naturally handles odd dimensions:

- Odd width: middle column is never swapped (stays in place)
- Odd height: middle row is never swapped (stays in place)

This eliminates the need for special-case handling of odd dimensions.

### 4. No Panics Across FFI

All error paths use early return instead of panic:

- `unwrap_or("")` for CStr conversion
- `match` with `Err` branch for JSON parsing
- No `.unwrap()` or `?` operator that could panic

### 5. SAFETY Documentation

Both `unsafe` blocks have descriptive `// SAFETY:` comments documenting the invariants:

1. CStr conversion: trusts the host to provide a valid null-terminated C string
2. Slice creation: trusts the host to provide a valid buffer of exactly `width * height * 4` bytes

---

## Architecture Notes

### Data Flow

```
CLI loads image -> plugin_loader calls process_image -> mirror_plugin modifies buffer -> CLI saves image
```

### Buffer Layout

- RGBA format: 4 bytes per pixel (R, G, B, A order)
- Row-major order: pixels stored left-to-right, top-to-bottom
- Pixel access: `index = (y * width + x) * 4`

### FFI Contract

```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
)
```

Matches the specification in `docs/vision.md`.

---

## Deferred Items and Technical Debt

### Completed as Planned

All Phase 5 tasks were completed without deferrals:

- 5.1 Define Params struct
- 5.2 Parse JSON params with serde
- 5.3 Implement horizontal flip
- 5.4 Implement vertical flip
- 5.5 Add test_images/mirror_params.json

### Known Limitations

1. **No performance benchmarks** - Large image performance was not measured
2. **No SIMD optimization** - Simple byte-by-byte swapping without vectorization
3. **Trust model** - Plugin trusts caller-provided dimensions and buffer validity

---

## How to Use

### Horizontal Flip (Mirror Effect)

```bash
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params <(echo '{"horizontal": true, "vertical": false}')
```

### Vertical Flip (Upside-Down)

```bash
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params <(echo '{"horizontal": false, "vertical": true}')
```

### Combined Flip (180-Degree Rotation)

```bash
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params <(echo '{"horizontal": true, "vertical": true}')
```

### Using the Default Params File

```bash
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params test_images/mirror_params.json
```

---

## Files Changed

| File | Change |
|------|--------|
| `mirror_plugin/src/lib.rs` | Full implementation replacing stub with flip logic and tests |
| `test_images/mirror_params.json` | Updated with actual parameters |

---

## Verification Checklist

All acceptance criteria were met:

- [x] `Params` struct with `horizontal` and `vertical` bool fields
- [x] Both fields default to `false` when omitted from JSON
- [x] Valid JSON parses successfully
- [x] Invalid JSON logs error and returns without panic
- [x] Empty JSON `{}` uses default values
- [x] Horizontal flip swaps pixels correctly within each row
- [x] Vertical flip swaps rows correctly from top to bottom
- [x] Combined flip produces 180-degree rotation
- [x] All unsafe blocks have `// SAFETY:` comments
- [x] No buffer access beyond `width * height * 4` bytes
- [x] 11 unit tests pass covering all scenarios
- [x] `cargo build` succeeds without warnings
- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] CLI produces correctly mirrored output image

---

## Next Steps

IF-5 completes the first image transformation plugin. The next iteration should implement:

1. **IF-6: Blur Plugin** - Implement weighted average blur logic in `blur_plugin`

---

## References

- `docs/prd/IF-5.prd.md` - Product Requirements Document
- `docs/plan/IF-5.md` - Implementation Plan
- `docs/tasklist/IF-5.md` - Task Checklist
- `reports/qa/IF-5.md` - QA Report
