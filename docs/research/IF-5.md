# IF-5: Mirror Plugin - Research Document

## Resolved Questions

User selected **"Use defaults"** - proceeding with documented requirements only. No additional implementation preferences or constraints were provided.

The PRD states there are no open questions as the requirements are clear from existing project documentation.

---

## Existing Endpoints and Contracts

### FFI Contract

All plugins must export this function (from `docs/vision.md` and `docs/conventions.md`):

```rust
#[no_mangle]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
)
```

**Parameters:**
| Parameter | Type | Description |
|-----------|------|-------------|
| `width` | `u32` | Image width in pixels |
| `height` | `u32` | Image height in pixels |
| `rgba_data` | `*mut u8` | Mutable pointer to RGBA buffer of size `width * height * 4` bytes |
| `params` | `*const c_char` | Null-terminated C string containing JSON parameters |

### Current Mirror Plugin State

Location: `mirror_plugin/src/lib.rs`

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

The plugin is a stub with no implementation. All parameters are prefixed with underscore indicating they are currently unused.

### Expected Parameters Format

From `docs/vision.md` and `docs/prd/IF-5.prd.md`:

```json
{"horizontal": true, "vertical": false}
```

Both `horizontal` and `vertical` are boolean values.

---

## Layers and Dependencies

### Plugin System Architecture

```
main.rs (CLI)
    |
    v
plugin_loader.rs (FFI bridge)
    |
    v
libmirror_plugin.dylib/so/dll (plugin)
```

### Plugin Loader Implementation

Location: `image_processor/src/plugin_loader.rs`

The plugin loader:
1. Constructs platform-specific library filename (`library_filename()`)
2. Loads the library using `libloading::Library::new()`
3. Gets the `process_image` symbol
4. Converts params to `CString`
5. Calls the FFI function with validated buffer

Key code:
```rust
type ProcessImageFn = unsafe extern "C" fn(u32, u32, *mut u8, *const c_char);

pub fn process(
    plugin_path: &Path,
    width: u32,
    height: u32,
    rgba_data: &mut [u8],
    params: &str,
) -> Result<()>
```

### Mirror Plugin Dependencies

From `mirror_plugin/Cargo.toml`:
- `serde = { version = "1", features = ["derive"] }` - For deserializing params
- `serde_json = "1"` - For JSON parsing
- `log = "0.4"` - For logging

Crate type: `cdylib` (C-compatible dynamic library)

---

## Patterns Used

### Plugin Structure Pattern (from `docs/vision.md`)

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
) {
    // SAFETY: params is a valid C string from the host
    let params_str = unsafe { CStr::from_ptr(params) }.to_str().unwrap_or("");

    let params: Params = match serde_json::from_str(params_str) {
        Ok(p) => p,
        Err(e) => {
            log::error!("Failed to parse params: {}", e);
            return; // Do nothing on invalid params
        }
    };

    let len = (width * height * 4) as usize;
    // SAFETY: rgba_data points to a valid buffer of len bytes, owned by the host
    let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, len) };

    // Process data in-place...
}
```

### Blur Plugin Reference

Location: `blur_plugin/src/lib.rs`

Currently also a no-op stub with identical structure to mirror_plugin. Not useful as a pattern for actual implementation, but confirms the expected function signature.

### Error Handling Pattern

- Parse errors: Log with `log::error!()` and return early
- Never panic across FFI boundary
- Use `unwrap_or("")` for C string conversion failures

---

## Technical Details

### Buffer Layout

- **Format**: RGBA with 4 bytes per pixel (Red, Green, Blue, Alpha order)
- **Layout**: Row-major order, starting from top-left corner
- **Size**: Always exactly `width * height * 4` bytes

### Pixel Access Formula

```rust
// For pixel at coordinates (x, y):
let pixel_index = (y * width + x) * 4;
// data[pixel_index]     = R
// data[pixel_index + 1] = G
// data[pixel_index + 2] = B
// data[pixel_index + 3] = A
```

### Row Access

```rust
// For row y:
let row_start = y * width * 4;
let row_end = row_start + width * 4;
let row = &mut data[row_start..row_end];
```

### Flip Algorithms

**Horizontal flip** (within each row):
- Swap pixel at x=0 with pixel at x=(width-1)
- Swap pixel at x=1 with pixel at x=(width-2)
- Continue until x reaches midpoint (width/2)

**Vertical flip** (swap rows):
- Swap row y=0 with row y=(height-1)
- Swap row y=1 with row y=(height-2)
- Continue until y reaches midpoint (height/2)

**Combined flip** (180-degree rotation):
- Apply both horizontal and vertical flips

---

## Limitations and Risks

### Constraints

1. **In-place modification only**: Cannot allocate new buffers; must swap pixels in the existing buffer
2. **No panics across FFI**: All errors must be handled gracefully with early return
3. **Buffer bounds**: Never read/write beyond `width * height * 4` bytes
4. **SAFETY comments required**: Every `unsafe` block needs documentation

### Risks

| Risk | Level | Mitigation |
|------|-------|------------|
| Off-by-one errors in loop bounds | Medium | Test with odd and even dimensions |
| Incorrect pixel offset calculation | Medium | Use formula `(y * width + x) * 4` consistently |
| Panic on invalid params | Low | Use pattern matching with early return |
| Buffer overflow | Low | Validate indices before access |

### Edge Cases to Consider

1. **1x1 image**: No swapping needed for either flip
2. **1xN or Nx1 images**: Only one flip direction has effect
3. **Odd dimensions**: Middle row/column should not be swapped with itself
4. **Both flags false**: No-op, return immediately
5. **Empty/invalid JSON**: Log error and return without modification

---

## Test Resources

### Existing Test Files

- `test_images/sample.png` - Sample test image
- `test_images/mirror_params.json` - Currently contains `{}` (empty object)

### Test Parameters to Create/Update

The `mirror_params.json` needs actual parameters:
```json
{"horizontal": true, "vertical": false}
```

### Test Strategy

1. **Unit tests**: Test flip functions with small known pixel arrays
2. **Integration tests**: Run full CLI with mirror_plugin
3. **Visual verification**: Compare output images manually

---

## New Technical Questions

None discovered during research. The requirements and implementation approach are clear:

1. Parse JSON params into a `Params` struct with `horizontal` and `vertical` bool fields
2. Convert raw pointer to mutable slice using `std::slice::from_raw_parts_mut`
3. Implement horizontal flip by swapping pixels within each row
4. Implement vertical flip by swapping entire rows
5. Apply both if both flags are true
6. Handle errors by logging and returning early

The implementation follows the established patterns from `docs/vision.md` and `docs/conventions.md`.
