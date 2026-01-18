# IF-5: Mirror Plugin - Implementation Plan

Status: PLAN_APPROVED

## Components

### 1. Params Struct (`mirror_plugin/src/lib.rs`)

A simple struct for deserializing JSON parameters:

```rust
#[derive(Deserialize)]
struct Params {
    #[serde(default)]
    horizontal: bool,
    #[serde(default)]
    vertical: bool,
}
```

Using `#[serde(default)]` allows omitted fields to default to `false`, handling empty JSON `{}` gracefully.

### 2. process_image Function (`mirror_plugin/src/lib.rs`)

The main FFI entry point that:
1. Parses the C string params to Rust `&str`
2. Deserializes JSON into `Params`
3. Converts raw pointer to mutable slice
4. Applies horizontal flip if requested
5. Applies vertical flip if requested

### 3. Flip Logic (inline in `process_image` or helper functions)

Simple inline loops for:
- **Horizontal flip**: Swap pixels within each row from edges toward center
- **Vertical flip**: Swap entire rows from top/bottom toward center

No separate modules needed - KISS principle.

### 4. Test Parameters File (`test_images/mirror_params.json`)

Update existing file with actual parameters:

```json
{"horizontal": true, "vertical": false}
```

---

## API Contract

### FFI Function Signature

```rust
#[no_mangle]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
)
```

### JSON Parameters Schema

```json
{
    "horizontal": <bool>,  // optional, defaults to false
    "vertical": <bool>     // optional, defaults to false
}
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `horizontal` | bool | false | Flip image horizontally (left-right swap) |
| `vertical` | bool | false | Flip image vertically (top-bottom swap) |

### Behavior Matrix

| horizontal | vertical | Result |
|------------|----------|--------|
| false | false | No-op (image unchanged) |
| true | false | Horizontal mirror |
| false | true | Vertical flip (upside-down) |
| true | true | 180-degree rotation |

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
mirror_plugin (process_image)
    |
    | 1. Parse params: CStr -> &str -> Params
    | 2. Convert rgba_data to &mut [u8] slice
    | 3. If horizontal: swap pixels in each row
    | 4. If vertical: swap rows top-to-bottom
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
let idx = (y * width + x) as usize * 4;
// data[idx..idx+4] contains [R, G, B, A]
```

**Horizontal flip (per row):**
```rust
for y in 0..height {
    for x in 0..width/2 {
        // swap pixel at (x, y) with pixel at (width-1-x, y)
    }
}
```

**Vertical flip (swap rows):**
```rust
for y in 0..height/2 {
    // swap row y with row (height-1-y)
    // Each row is width*4 bytes
}
```

---

## NFR (Non-Functional Requirements)

### Performance

- **Time complexity**: O(width * height) - each pixel touched at most once per flip
- **Space complexity**: O(1) - in-place swapping with no temporary buffers beyond stack variables
- **Expected performance**: Sub-millisecond for typical images (under 10MP)

### Safety

1. All `unsafe` blocks must have `// SAFETY:` comments
2. No buffer access beyond `width * height * 4` bytes
3. No panics across FFI boundary - all errors handled with early return
4. Use `unwrap_or("")` for C string conversion, not `.unwrap()`

### Code Quality

- `cargo build` with no warnings
- `cargo clippy -- -D warnings` passes
- `cargo fmt --check` passes
- `cargo test -p mirror_plugin` passes

---

## Risks

| Risk | Level | Mitigation |
|------|-------|------------|
| Off-by-one errors in loop bounds | Medium | Test with 1x1, odd dimensions, and typical images |
| Incorrect pixel offset calculation | Medium | Use consistent formula `(y * width + x) * 4` |
| Panic on invalid JSON | Low | Use `match` with `Err` branch returning early |
| Buffer overflow | Low | Bounds are statically correct when using `width/2` and `height/2` |

### Edge Cases

1. **1x1 image**: Both flips are no-ops (single pixel, nothing to swap)
2. **1xN image**: Horizontal flip is no-op; vertical flip works normally
3. **Nx1 image**: Vertical flip is no-op; horizontal flip works normally
4. **Odd dimensions**: Middle row/column naturally excluded by integer division
5. **Both flags false**: Early return, no processing needed
6. **Empty JSON `{}`**: Serde defaults both to false, treated as no-op
7. **Invalid JSON**: Log error and return without modification

---

## Open Questions

None. The requirements are fully specified:

- FFI contract defined in `docs/vision.md` and `docs/conventions.md`
- JSON parameter format documented
- Flip algorithms are straightforward swaps
- Error handling pattern established (log and return early)
- All dependencies already configured in `mirror_plugin/Cargo.toml`

---

## Implementation Tasks

Based on PRD section "Phase 5 Tasks":

1. **5.1** Define `Params` struct with `horizontal` and `vertical` bool fields
2. **5.2** Parse JSON params with serde (handle errors gracefully)
3. **5.3** Implement horizontal flip (swap pixels within each row)
4. **5.4** Implement vertical flip (swap rows)
5. **5.5** Update `test_images/mirror_params.json` with actual parameters

### Verification

- `cargo build` succeeds
- `cargo test -p mirror_plugin` passes
- `cargo clippy -- -D warnings` passes
- Manual test: run CLI with mirror_plugin and verify output image
