# IF-6: Blur Plugin - Implementation Plan

Status: PLAN_APPROVED

## Components

### 1. Params Struct (`blur_plugin/src/lib.rs`)

A simple struct for deserializing JSON parameters:

```rust
#[derive(Deserialize)]
struct Params {
    #[serde(default = "default_radius")]
    radius: u32,
    #[serde(default = "default_iterations")]
    iterations: u32,
}

fn default_radius() -> u32 { 1 }
fn default_iterations() -> u32 { 1 }
```

Using `#[serde(default)]` with helper functions provides sensible defaults when fields are omitted.

### 2. process_image Function (`blur_plugin/src/lib.rs`)

The main FFI entry point that:
1. Parses the C string params to Rust `&str`
2. Deserializes JSON into `Params`
3. Early returns if radius or iterations is 0 (no-op)
4. Converts raw pointer to mutable slice
5. Applies blur algorithm for specified number of iterations

### 3. Blur Algorithm (inline in `blur_plugin/src/lib.rs`)

Weighted average blur implementation:
- For each pixel, compute weighted average of neighbors within radius
- Weight formula: `1.0 / (distance + 1.0)` (inverse distance weighting)
- Use temporary buffer to avoid reading modified pixels
- Copy results back after processing all pixels
- Repeat for each iteration

No separate modules needed - KISS principle.

### 4. Test Parameters File (`test_images/blur_params.json`)

Create new file with test parameters:

```json
{"radius": 3, "iterations": 1}
```

---

## API Contract

### FFI Function Signature

```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
)
```

### JSON Parameters Schema

```json
{
    "radius": <u32>,     // optional, defaults to 1
    "iterations": <u32>  // optional, defaults to 1
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `radius` | u32 | 1 | Blur radius - pixels within this distance are averaged |
| `iterations` | u32 | 1 | Number of times to apply the blur effect |

### Behavior Matrix

| radius | iterations | Result |
|--------|------------|--------|
| 0 | any | No-op (image unchanged) |
| any | 0 | No-op (image unchanged) |
| 1 | 1 | Subtle blur (3x3 kernel) |
| 3 | 1 | Medium blur (7x7 kernel) |
| 5 | 3 | Strong blur (multiple passes) |

---

## Data Flows

```
CLI (main.rs)
    |
    | load image, read params file
    v
plugin_loader.rs
    |
    | call process_image(width, height, rgba_data, params)
    v
blur_plugin (process_image)
    |
    | 1. Parse params: CStr -> &str -> Params
    | 2. Early return if radius=0 or iterations=0
    | 3. Convert rgba_data to &mut [u8] slice
    | 4. For each iteration:
    |    a. Allocate temporary buffer (same size)
    |    b. For each pixel (cx, cy):
    |       - For each neighbor (x, y) within radius:
    |         - Calculate distance from center
    |         - Accumulate weighted color values
    |       - Store weighted average in temp buffer
    |    c. Copy temp buffer back to original
    |
    v
Buffer modified in-place
    |
    v
plugin_loader.rs returns
    |
    v
CLI saves modified image
```

### Buffer Access Pattern

**Pixel at (x, y):**
```rust
let idx = (y * width + x) * 4;
// data[idx..idx+4] contains [R, G, B, A]
```

**Neighbor iteration within radius:**
```rust
for dy in -(radius as i32)..=(radius as i32) {
    for dx in -(radius as i32)..=(radius as i32) {
        let nx = cx as i32 + dx;
        let ny = cy as i32 + dy;
        // Check bounds: 0 <= nx < width && 0 <= ny < height
        // Calculate distance: sqrt(dx^2 + dy^2) as f64
        // Weight: 1.0 / (distance + 1.0)
    }
}
```

**Weight calculation:**
```rust
let distance = ((dx * dx + dy * dy) as f64).sqrt();
let weight = 1.0 / (distance + 1.0);
```

---

## NFR (Non-Functional Requirements)

### Performance

- **Time complexity**: O(width * height * radius^2 * iterations) per iteration
- **Space complexity**: O(width * height * 4) - temporary buffer for intermediate results
- **Expected performance**: Acceptable for typical images with reasonable radius values
- **Memory allocation**: Temporary buffer allocated inside plugin, freed before return

### Safety

1. All `unsafe` blocks must have `// SAFETY:` comments
2. No buffer access beyond `width * height * 4` bytes
3. No panics across FFI boundary - all errors handled with early return
4. Use `unwrap_or("")` for C string conversion, not `.unwrap()`
5. Temporary buffer fully owned by plugin, no cross-FFI memory ownership

### Code Quality

- `cargo build` with no warnings
- `cargo clippy -- -D warnings` passes
- `cargo fmt --check` passes
- `cargo test -p blur_plugin` passes

### Numerical Precision

- Use f64 for intermediate weight and sum calculations
- Round final values to u8 using `.round() as u8`
- Clamp values to 0-255 range (automatic with weighted average of valid pixels)

---

## Risks

| Risk | Level | Mitigation |
|------|-------|------------|
| Performance with large radius | Medium | Document expected behavior; no optimization required per PRD |
| Memory allocation for temp buffer | Low | Single allocation per iteration; freed automatically |
| Edge pixel handling | Medium | Carefully check bounds; only average valid neighbors |
| Numerical precision errors | Low | Use f64 for intermediate calculations |
| Division by zero in weight | Low | Formula `1.0 / (distance + 1.0)` prevents this |

### Edge Cases

1. **1x1 image**: Each pixel averages only itself (weight 1.0); result unchanged
2. **Radius of 0**: Early return, no processing needed
3. **Iterations of 0**: Early return, no processing needed
4. **Radius larger than image**: Works correctly; only valid neighbors are averaged
5. **Empty JSON `{}`**: Serde defaults to radius=1, iterations=1
6. **Invalid JSON**: Log error and return without modification
7. **Corner pixels**: Average fewer neighbors (only valid ones within bounds)
8. **Edge pixels**: Average neighbors that exist (clamp to image bounds)

---

## Algorithm Details

### Weighted Average Blur

The algorithm specification from `docs/idea.md`:
> The new pixel color is equal to the weighted average of the colors of the pixels within the blur radius, where the weight is the distance between the center pixel and the current pixel.

**Implementation interpretation:**
- Use inverse distance weighting (closer pixels have more influence)
- Weight formula: `weight = 1.0 / (distance + 1.0)`
- Distance is Euclidean: `sqrt(dx^2 + dy^2)`
- Square kernel region (iterate over square, use circular-ish weighting)

**Pseudocode:**
```
for each iteration:
    create temp_buffer[width * height * 4]
    for cy in 0..height:
        for cx in 0..width:
            weight_sum = 0.0
            color_sum = [0.0, 0.0, 0.0, 0.0]  // R, G, B, A

            for dy in -radius..=radius:
                for dx in -radius..=radius:
                    nx = cx + dx
                    ny = cy + dy
                    if nx in bounds and ny in bounds:
                        distance = sqrt(dx^2 + dy^2)
                        weight = 1.0 / (distance + 1.0)
                        weight_sum += weight
                        for channel in 0..4:
                            color_sum[channel] += weight * pixel[nx, ny][channel]

            for channel in 0..4:
                temp_buffer[cx, cy][channel] = (color_sum[channel] / weight_sum).round() as u8

    copy temp_buffer to rgba_data
```

---

## Open Questions

None. The requirements are fully specified:

- FFI contract defined in `docs/vision.md` and `docs/conventions.md`
- JSON parameter format documented (`{"radius": u32, "iterations": u32}`)
- Algorithm specified in `docs/idea.md` (weighted average based on distance)
- Weight formula clarified in research: `1.0 / (distance + 1.0)`
- Error handling pattern established (log and return early)
- All dependencies already configured in `blur_plugin/Cargo.toml`

---

## Implementation Tasks

Based on PRD section "Phase 6 Tasks":

1. **6.1** Define `Params` struct with `radius` and `iterations` u32 fields
2. **6.2** Parse JSON params with serde (handle errors gracefully)
3. **6.3** Implement weighted average blur algorithm with temporary buffer
4. **6.4** Support multiple iterations (repeat blur application)
5. **6.5** Create `test_images/blur_params.json` with test parameters

### Unit Tests

Following the mirror_plugin pattern:

1. **test_basic_blur**: Verify blur modifies pixels (not identical to input)
2. **test_blur_smoothing**: Verify sharp edge becomes smoother
3. **test_zero_radius**: Verify no modification when radius=0
4. **test_zero_iterations**: Verify no modification when iterations=0
5. **test_multiple_iterations**: Verify stronger blur with more iterations
6. **test_1x1_image**: Verify single pixel image handled correctly
7. **test_invalid_json**: Verify early return without modification
8. **test_empty_json**: Verify defaults applied (radius=1, iterations=1)

### Verification

- `cargo build` succeeds
- `cargo test -p blur_plugin` passes
- `cargo clippy -- -D warnings` passes
- Manual test: run CLI with blur_plugin and verify output image is blurred
