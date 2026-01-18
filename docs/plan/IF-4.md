# IF-4: Plugin Loader - Implementation Plan

**Status:** PLAN_APPROVED

## Overview

Implement the plugin loader module (`plugin_loader.rs`) to dynamically load plugin libraries via FFI and call the `process_image` function. This module isolates all unsafe FFI code from the rest of the application, providing a safe Rust interface for plugin invocation. The implementation supports cross-platform library loading (macOS, Linux, Windows) using the `libloading` crate.

## Components

### 1. plugin_loader.rs Module

**Location:** `image_processor/src/plugin_loader.rs`

**Components:**

| Function | Purpose | Visibility |
|----------|---------|------------|
| `process()` | Main entry point - loads library, gets symbol, calls function | `pub` |
| `library_filename()` | Constructs platform-specific library filename | internal |

### 2. Type Definitions

```rust
/// Function signature for the plugin's process_image function
type ProcessImageFn = unsafe extern "C" fn(u32, u32, *mut u8, *const c_char);
```

## API Contract

### Public Interface

```rust
/// Loads a plugin library and calls its process_image function.
///
/// # Arguments
/// * `plugin_path` - Full path to the plugin library file
/// * `width` - Image width in pixels
/// * `height` - Image height in pixels
/// * `rgba_data` - Mutable slice of RGBA pixel data (length must be width * height * 4)
/// * `params` - JSON parameter string to pass to the plugin
///
/// # Errors
/// Returns an error if:
/// - The library file cannot be loaded
/// - The `process_image` symbol is not found
/// - The params string contains null bytes (invalid for C string)
pub fn process(
    plugin_path: &Path,
    width: u32,
    height: u32,
    rgba_data: &mut [u8],
    params: &str,
) -> Result<()>
```

### Internal Interface

```rust
/// Constructs the platform-specific library filename from a plugin name.
///
/// # Examples
/// - macOS: "mirror_plugin" -> "libmirror_plugin.dylib"
/// - Linux: "mirror_plugin" -> "libmirror_plugin.so"
/// - Windows: "mirror_plugin" -> "mirror_plugin.dll"
fn library_filename(plugin_name: &str) -> String
```

## Data Flows

### Plugin Loading and Execution Flow

```
main.rs                         plugin_loader.rs
   |                                   |
   |  1. Read params file              |
   |  2. Get mut reference to rgba_data|
   |  3. Build plugin library path     |
   |                                   |
   +--- process(path, w, h, data, params) --->
                                       |
                                  4. Load library (unsafe)
                                       |
                                  5. Get symbol (unsafe)
                                       |
                                  6. Convert params to CString
                                       |
                                  7. Call process_image (unsafe)
                                       |
                                  8. Library dropped (RAII)
                                       |
   <--------- Ok(()) -----------------+
   |
   9. Reconstruct image from modified data
   10. Save output image
```

### Memory Ownership

- `rgba_data`: Owned by `main.rs`, borrowed mutably by plugin loader, modified in-place by plugin
- `CString params`: Created and owned by plugin loader, pointer valid for duration of FFI call
- `Library`: Owned by plugin loader, dropped after FFI call completes (RAII)

## Non-Functional Requirements (NFR)

### Safety

1. **Unsafe Isolation**: All unsafe code confined to `plugin_loader.rs`
2. **SAFETY Comments**: Every unsafe block documented with:
   - What invariants must hold
   - Why those invariants are satisfied
3. **No Panics Across FFI**: Errors returned via `Result`, not panics
4. **Buffer Bounds**: Document that plugins must not exceed `width * height * 4` bytes

### Error Handling

1. **Library Not Found**: Clear error message with expected path
2. **Symbol Not Found**: Error indicating `process_image` symbol missing
3. **CString Conversion**: Handle null bytes in params string
4. **Use anyhow::Context**: Add context to all error paths

### Logging

1. `debug!` - Library path, params content, dimensions
2. `info!` - Plugin loading, execution start/complete
3. `error!` - Failures during loading or symbol resolution

### Performance

- Library loaded once per invocation
- No unnecessary allocations
- CString created once before FFI call

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Plugin panics across FFI | Undefined behavior, crash | Document that plugins must not panic; add logging before/after calls for debugging |
| Memory corruption from malicious plugin | Data corruption, security | Trust model assumes first-party plugins; document bounds requirements |
| Symbol type mismatch | Undefined behavior | Document exact signature; type alias enforces signature at Rust level |
| Library load fails on missing dependencies | Confusing error message | Add context to error with full path |
| CString null byte in params | Error on conversion | CString::new returns Result; propagate with context |

## Implementation Steps

### Step 1: Create module structure

1. Replace stub in `plugin_loader.rs` with module imports and type definitions
2. Add `mod plugin_loader;` to `main.rs` (if not present)

### Step 2: Implement library_filename()

1. Use `cfg!(target_os = "...")` for platform detection
2. Return appropriate format: `lib{name}.dylib`, `lib{name}.so`, or `{name}.dll`

### Step 3: Implement process() function

1. Log library path and params (debug level)
2. Load library with `Library::new()` - add SAFETY comment
3. Get `process_image` symbol with `lib.get()` - add SAFETY comment
4. Convert params to CString
5. Call function - add SAFETY comment
6. Log completion (info level)
7. Return Ok(())

### Step 4: Integrate with main.rs

1. Add `mod plugin_loader;` declaration
2. Read params file: `let params = std::fs::read_to_string(&args.params)?;`
3. Make rgba_data mutable: `let mut rgba_data: Vec<u8> = img.into_raw();`
4. Build library path using `library_filename()`
5. Call `plugin_loader::process()` before image reconstruction
6. Add logging for plugin invocation

### Step 5: Add tests

1. Test that loads a real plugin (mirror_plugin) and calls process_image
2. Test error case for missing library
3. Test library_filename() for each platform

## Files to Modify/Create

| File | Action | Changes |
|------|--------|---------|
| `image_processor/src/plugin_loader.rs` | Modify | Replace stub with full implementation |
| `image_processor/src/main.rs` | Modify | Add mod declaration, params reading, plugin invocation |

## Out of Scope

1. **Plugin implementation**: Mirror/blur plugin logic (IF-5, IF-6)
2. **Plugin discovery**: Auto-discovery of available plugins
3. **Plugin versioning**: ABI version checking
4. **Concurrent execution**: Thread safety for parallel plugin calls
5. **Plugin sandboxing**: Security isolation of plugins
6. **Custom error types**: Using anyhow only, no plugin-specific errors

## Acceptance Criteria

1. **Module exists**: `plugin_loader.rs` contains the `process()` function
2. **Safe interface**: `process()` returns `anyhow::Result<()>` with no unsafe code exposed
3. **Unsafe isolated**: All unsafe blocks contained within `plugin_loader.rs`
4. **SAFETY documented**: Every unsafe block has a `// SAFETY:` comment
5. **Platform support**: `library_filename()` works for macOS, Linux, Windows
6. **Symbol loading**: Successfully loads `process_image` from valid plugins
7. **Error messages**: Returns descriptive errors for missing library/symbol
8. **CString handling**: Params converted without memory leaks
9. **Integration complete**: `main.rs` calls plugin before saving image
10. **Test coverage**: At least one test that loads a plugin and calls process_image
11. **No panics**: Function never panics; all errors returned via Result

## Alternatives Considered

### 1. Using dlopen directly instead of libloading

**Rejected because:**
- libloading provides safe abstractions and is the standard Rust approach
- Cross-platform support built-in
- RAII cleanup handled automatically
- Already a project dependency

### 2. Returning Result from plugins

**Rejected because:**
- FFI Result types are complex and non-standard
- Simpler to have plugins log errors and return early
- Matches existing plugin stub design
- Host has no meaningful recovery from plugin errors anyway

### 3. Separate library_filename as a public utility function

**Rejected because:**
- Only needed internally by plugin_loader
- KISS principle - no premature abstraction
- Can be made public later if needed
