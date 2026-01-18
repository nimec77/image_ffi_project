# IF-4: Plugin Loader - Research Document

## Resolved Questions

**User Preference:** Use defaults - proceed with documented requirements only.

The PRD for IF-4 had no open questions. All requirements are clearly defined in `docs/vision.md` and `docs/idea.md`.

---

## Related Modules and Services

### Current Code Structure

| File | Purpose | Current State |
|------|---------|---------------|
| `image_processor/src/main.rs` | CLI entry point, image I/O | Implemented - loads/saves images, parses args |
| `image_processor/src/plugin_loader.rs` | FFI code isolation | Stub only - `// Plugin loader stub - FFI code will be implemented here` |
| `mirror_plugin/src/lib.rs` | Mirror transform plugin | No-op stub with correct FFI signature |
| `blur_plugin/src/lib.rs` | Blur effect plugin | No-op stub with correct FFI signature |

### Dependencies Already Configured

`image_processor/Cargo.toml` already includes:
```toml
libloading = "0.9.0"
anyhow = "1"
log = "0.4"
env_logger = "0.11"
```

Both plugins are configured as `cdylib` crates:
```toml
[lib]
crate-type = ["cdylib"]
```

---

## Current Endpoints and Contracts

### FFI Contract (Defined in Both Plugins)

Both `mirror_plugin/src/lib.rs` and `blur_plugin/src/lib.rs` export:

```rust
use std::ffi::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn process_image(
    _width: u32,
    _height: u32,
    _rgba_data: *mut u8,
    _params: *const c_char,
)
```

**Note:** The plugins use `#[unsafe(no_mangle)]` which is the Rust 2024 edition syntax for unsafe extern functions.

### Expected Plugin Loader Interface

From `docs/vision.md`, the plugin loader should expose:

```rust
pub fn process(
    plugin_path: &Path,
    width: u32,
    height: u32,
    rgba_data: &mut [u8],
    params: &str
) -> Result<()>
```

### CLI Arguments (Already Implemented)

```rust
struct Args {
    input: PathBuf,       // --input
    output: PathBuf,      // --output
    plugin: String,       // --plugin (name without extension)
    params: PathBuf,      // --params (JSON file path)
    plugin_path: PathBuf, // --plugin-path (default: "target/debug")
}
```

---

## Patterns Used in the Project

### Error Handling Pattern

From `main.rs`:
```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    // ...
    let img = image::open(&args.input)
        .with_context(|| format!("Failed to load image: {}", args.input.display()))?;
    // ...
}
```

**Pattern:** Use `anyhow::Result`, `.with_context()` for descriptive errors, `?` for propagation.

### Logging Pattern

From `main.rs`:
```rust
use log::{debug, info};

fn main() -> Result<()> {
    env_logger::init();
    // ...
    debug!("Loaded image: {}x{} ({} bytes)", width, height, rgba_data.len());
    info!("Saved image to: {}", args.output.display());
}
```

**Pattern:** Initialize `env_logger` once in main, use `debug!` for details, `info!` for major steps, `error!` for failures.

### Image Data Flow Pattern

From `main.rs`:
```rust
let img = image::open(&args.input)?.into_rgba8();
let (width, height) = img.dimensions();
let rgba_data: Vec<u8> = img.into_raw();
// ... process ...
let output_img = RgbaImage::from_raw(width, height, rgba_data)
    .expect("Buffer size mismatch - should never happen with unchanged data");
output_img.save(&args.output)?;
```

**Pattern:** Image -> RGBA8 -> dimensions + raw bytes -> process -> reconstruct -> save.

---

## Platform-Specific Library Naming

From the PRD and `docs/vision.md`:

| Platform | Library Filename Pattern |
|----------|-------------------------|
| macOS    | `lib{plugin_name}.dylib` |
| Linux    | `lib{plugin_name}.so` |
| Windows  | `{plugin_name}.dll` |

**Implementation approach:**
```rust
fn library_filename(plugin_name: &str) -> String {
    if cfg!(target_os = "macos") {
        format!("lib{}.dylib", plugin_name)
    } else if cfg!(target_os = "linux") {
        format!("lib{}.so", plugin_name)
    } else if cfg!(target_os = "windows") {
        format!("{}.dll", plugin_name)
    } else {
        // Fallback to Linux-style
        format!("lib{}.so", plugin_name)
    }
}
```

---

## libloading Usage Pattern

From libloading 0.9.0 documentation:

```rust
use libloading::{Library, Symbol};
use std::ffi::CString;
use std::os::raw::c_char;

// Type alias for the function signature
type ProcessImageFn = unsafe extern "C" fn(u32, u32, *mut u8, *const c_char);

// Load the library
let lib = unsafe { Library::new(library_path) }?;

// Get the symbol
let func: Symbol<ProcessImageFn> = unsafe { lib.get(b"process_image\0") }?;

// Call the function
let c_params = CString::new(params)?;
unsafe { func(width, height, rgba_data.as_mut_ptr(), c_params.as_ptr()) };

// Library is dropped automatically (RAII)
```

---

## Safety Requirements for Unsafe Blocks

Every `unsafe` block must have a `// SAFETY:` comment. Required blocks:

### 1. Library Loading
```rust
// SAFETY: The library path points to a valid cdylib built from trusted first-party code.
// The library follows the expected ABI and will be properly unloaded when dropped.
let lib = unsafe { Library::new(&library_path) }?;
```

### 2. Symbol Retrieval
```rust
// SAFETY: The "process_image" symbol exists in valid plugins with the signature
// extern "C" fn(u32, u32, *mut u8, *const c_char). Type mismatch would cause UB.
let func: Symbol<ProcessImageFn> = unsafe { lib.get(b"process_image\0") }?;
```

### 3. Function Invocation
```rust
// SAFETY:
// - rgba_data is a valid mutable slice of exactly width * height * 4 bytes
// - c_params is a valid null-terminated CString
// - The plugin will only modify data within bounds
// - The plugin will not panic across the FFI boundary
unsafe { func(width, height, rgba_data.as_mut_ptr(), c_params.as_ptr()) };
```

---

## Integration Points

### Changes Required to main.rs

1. Add `mod plugin_loader;` declaration
2. Read params file content: `let params = std::fs::read_to_string(&args.params)?;`
3. Make `rgba_data` mutable: `let mut rgba_data: Vec<u8> = img.into_raw();`
4. Call plugin loader before saving:
   ```rust
   let plugin_lib_path = args.plugin_path.join(library_filename(&args.plugin));
   plugin_loader::process(&plugin_lib_path, width, height, &mut rgba_data, &params)?;
   ```

---

## Limitations and Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Plugin crashes/panics | Application crash, UB | Document that plugins must not panic; log before/after calls |
| Memory corruption | Data corruption, security | Trust plugins as first-party; validate buffer size |
| Symbol type mismatch | UB at runtime | Document exact signature; no runtime validation possible |
| Library not found | Error on plugin load | Return descriptive error with expected path |
| CString null bytes | Error on conversion | `CString::new()` returns Result; use `?` |
| Invalid UTF-8 in params | Conversion error | `CString::new()` handles this; return error |

### Limitations

1. **No runtime ABI validation** - Cannot verify plugin was compiled with compatible signature
2. **No plugin versioning** - No way to check plugin version compatibility
3. **Trust model** - Plugins are trusted first-party code; no sandboxing
4. **Single-threaded** - No concurrent plugin invocation considerations

---

## New Technical Questions

None discovered during research. The PRD, `docs/vision.md`, and `docs/conventions.md` provide complete guidance for implementation.

---

## Implementation Checklist (From PRD)

- [ ] Create `plugin_loader.rs` with `pub fn process()` returning `anyhow::Result<()>`
- [ ] Implement platform-specific library filename construction
- [ ] Load library with `libloading::Library::new()`
- [ ] Get `process_image` symbol with `lib.get()`
- [ ] Convert params to `CString`
- [ ] Call function with SAFETY comments on all unsafe blocks
- [ ] Add logging: debug for path/params, info for major steps, error for failures
- [ ] Return descriptive errors for missing library or symbol
- [ ] Write at least one test that loads a plugin and calls process_image
