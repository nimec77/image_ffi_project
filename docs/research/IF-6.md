# IF-6: Blur Plugin - Research Document

## Resolved Questions

The user has confirmed to use defaults (documented requirements only). No open questions were present in the PRD.

**Implementation approach:**
- Follow the weighted average blur algorithm as specified in `docs/idea.md`
- Use the parameter format `{"radius": u32, "iterations": u32}` from `docs/vision.md`
- Model implementation after the mirror plugin reference pattern

---

## Existing Endpoints and Contracts

### FFI Contract

All plugins must export this function signature (from `docs/vision.md`):

```rust
#[no_mangle]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
)
```

The plugin loader (`image_processor/src/plugin_loader.rs`) defines the function type as:
```rust
type ProcessImageFn = unsafe extern "C" fn(u32, u32, *mut u8, *const c_char);
```

### Parameter Format

JSON structure for blur plugin (from `docs/vision.md`):
```json
{"radius": 5, "iterations": 2}
```

- `radius` - blur radius (u32)
- `iterations` - number of times to apply the blur (u32)

### Pixel Data Format

- **Format**: RGBA with 4 bytes per pixel (R, G, B, A order)
- **Buffer layout**: Row-major order, starting from top-left corner
- **Pixel access formula**: `pixel_index = (y * width + x) * 4`
- **Total buffer size**: `width * height * 4` bytes

---

## Layers and Dependencies

### Plugin Structure

The blur plugin crate is already set up at `blur_plugin/`:

```
blur_plugin/
├── Cargo.toml    # cdylib, dependencies configured
└── src/lib.rs    # Current stub implementation
```

### Current State of blur_plugin/src/lib.rs

The file contains only a no-op stub:
```rust
use std::ffi::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn process_image(
    _width: u32,
    _height: u32,
    _rgba_data: *mut u8,
    _params: *const c_char,
) {
    // No-op stub
}
```

### Available Dependencies (blur_plugin/Cargo.toml)

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
log = "0.4"
```

All required dependencies are already configured. No additional crates are permitted.

---

## Patterns Used

### Reference: mirror_plugin/src/lib.rs

The mirror plugin provides the canonical pattern for implementing plugins:

1. **Imports**:
   ```rust
   use log::error;
   use serde::Deserialize;
   use std::ffi::{CStr, c_char};
   ```

2. **Params struct with serde**:
   ```rust
   #[derive(Deserialize)]
   struct Params {
       #[serde(default)]
       horizontal: bool,
       #[serde(default)]
       vertical: bool,
   }
   ```

3. **FFI function structure**:
   - `#[unsafe(no_mangle)]` attribute (Rust 2024 edition syntax)
   - Parse params from C string with SAFETY comment
   - Deserialize JSON with error logging on failure
   - Early return on parse error (no modification)
   - Create mutable slice from raw pointer with SAFETY comment
   - Process data in-place

4. **SAFETY comments** are required for every unsafe block:
   - Params pointer validity
   - Buffer pointer validity and bounds

5. **Test structure**:
   - Helper function `call_process_image` wrapping unsafe call
   - Test image creation helpers
   - Tests for valid input, edge cases, and error cases

### Edge Handling Pattern

The blur algorithm must handle image boundaries. Unlike the mirror plugin which uses in-place swaps, blur needs to:
- Only average pixels within valid bounds
- Handle corners and edges where fewer neighbors exist

---

## Limitations and Risks

### Performance Considerations

1. **Algorithm complexity**: O(n * r^2) where n is pixel count and r is radius
   - Large images with large radii will be slow
   - Acceptable per PRD; no optimization required

2. **Memory for intermediate buffer**: Blur cannot modify pixels in-place without a temporary buffer because:
   - Reading neighbor values after some are modified produces incorrect results
   - PRD acknowledges this: "May need temporary buffer to avoid reading modified pixels"
   - Solution: Allocate temporary buffer, copy results back

### Safety Constraints

1. **No panics across FFI boundary**: Must catch all errors gracefully
2. **Buffer bounds**: Never access beyond `width * height * 4` bytes
3. **No memory leaks**: Any temporary allocations must be freed before return

### Numerical Precision

1. **Intermediate calculations**: Use f64 for weight and average computations
2. **Final values**: Round to u8 for pixel values (clamp to 0-255)

### Alpha Channel

Per PRD assumption: "Blur all four channels (R, G, B, A) uniformly for simplicity"

### Edge Cases

1. **Radius of 0**: Should result in no change (each pixel averages only itself)
2. **Iterations of 0**: Should result in no change (blur applied zero times)
3. **Invalid JSON**: Log error and return without modification (mirror plugin pattern)
4. **1x1 image**: Should work correctly (pixel averages only itself)

---

## Algorithm Specification

From `docs/idea.md`:

> The new pixel color is equal to the weighted average of the colors of the pixels within the blur radius, where the weight is the distance between the center pixel and the current pixel.

**Implementation approach**:
1. For each pixel (cx, cy):
   - Iterate over all pixels (x, y) where `|x - cx| <= radius` and `|y - cy| <= radius`
   - Calculate distance: `d = sqrt((x - cx)^2 + (y - cy)^2)`
   - Weight can be `1 / (d + 1)` or `radius - d + 1` (inversely related to distance)
   - Sum weighted values, sum weights
   - New value = weighted_sum / total_weight

2. Use a temporary buffer to store results
3. After processing all pixels, copy temporary buffer back
4. Repeat for each iteration

**Weight formula consideration**:
- The PRD states "weight is the distance" but also "inversely related to distance"
- Standard weighted blur uses inverse distance (closer pixels have more weight)
- Suggested formula: `weight = 1.0 / (distance + 1.0)` to avoid division by zero

---

## Test Files Required

Need to create `test_images/blur_params.json` with content:
```json
{"radius": 3, "iterations": 1}
```

---

## New Technical Questions

These questions arose during research but do not block implementation:

1. **Weight formula precision**: The exact weight formula is not explicitly defined. Using `1.0 / (distance + 1.0)` seems reasonable, but alternatives like `max(0, radius - distance)` are also valid. The PRD says "inversely related to distance" which supports the first approach.

2. **Circular vs square kernel**: Should the blur only consider pixels within a circular radius (Euclidean distance <= radius) or all pixels in a square kernel (max of |dx|, |dy| <= radius)? Standard blur typically uses a square kernel with circular weighting.

3. **Distance calculation**: Should distance be Euclidean (`sqrt(dx^2 + dy^2)`) or Manhattan (`|dx| + |dy|`)? Euclidean is more common for visual blur effects.

**Resolution**: Proceed with:
- Square kernel (iterate over square region)
- Euclidean distance for weight calculation
- Weight formula: `1.0 / (distance + 1.0)`

These choices align with standard blur implementations and the PRD's "distance" terminology.

---

## Summary

The blur plugin implementation is well-defined:

1. **FFI contract**: Matches existing pattern from mirror_plugin
2. **Dependencies**: All required (serde, serde_json, log) are configured
3. **Algorithm**: Weighted average blur with configurable radius and iterations
4. **Reference**: mirror_plugin provides a complete implementation pattern
5. **Test structure**: Follow mirror_plugin's test organization

Key implementation tasks:
- Define `Params` struct with `radius: u32` and `iterations: u32`
- Parse JSON params with serde (with `#[serde(default)]` for resilience)
- Implement weighted average blur algorithm with temporary buffer
- Support multiple iterations by applying blur repeatedly
- Add comprehensive unit tests
- Create `test_images/blur_params.json`
