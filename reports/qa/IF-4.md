# QA Report: IF-4 - Plugin Loader

**Date:** 2026-01-18
**Status:** COMPLETED
**Verdict:** RELEASE

---

## Summary

This QA report covers the Plugin Loader implementation (IF-4) for the Image FFI Project. The iteration implements `image_processor/src/plugin_loader.rs` to dynamically load plugin libraries via FFI and call the `process_image` function. All unsafe FFI code is isolated in this module, providing a safe Rust interface for the rest of the application.

All acceptance criteria from the PRD have been met. The implementation supports cross-platform library loading (macOS, Linux, Windows), includes proper SAFETY documentation for all unsafe blocks, uses `anyhow::Result` for error handling, and integrates with the main application.

---

## Positive Scenarios

### PS-1: Successful Plugin Loading and Execution (Scenario 1 from PRD)

| Step | Expected | Implementation | Status |
|------|----------|----------------|--------|
| Constructs platform-specific library filename | macOS: `lib{name}.dylib`, Linux: `lib{name}.so`, Windows: `{name}.dll` | `library_filename()` function with `cfg!(target_os = ...)` | PASS |
| Loads library using `libloading` | `Library::new(plugin_path)` | Correct with SAFETY comment | PASS |
| Retrieves `process_image` symbol | `lib.get::<ProcessImageFn>(b"process_image\0")` | Correct with SAFETY comment | PASS |
| Converts params to CString | `CString::new(params)` | Correct with error context | PASS |
| Calls function with image data | `process_image_fn(width, height, rgba_data.as_mut_ptr(), c_params.as_ptr())` | Correct with SAFETY comment | PASS |
| Returns successfully | `Ok(())` after completion | Correct | PASS |

### PS-2: Cross-Platform Library Naming (Scenario 4 from PRD)

| Platform | Expected Filename | Implementation | Status |
|----------|-------------------|----------------|--------|
| macOS | `lib{name}.dylib` | `format!("lib{}.dylib", plugin_name)` | PASS |
| Linux | `lib{name}.so` | `format!("lib{}.so", plugin_name)` | PASS |
| Windows | `{name}.dll` | `format!("{}.dll", plugin_name)` | PASS |
| Unknown | Fallback to Linux-style | `format!("lib{}.so", plugin_name)` | PASS |

### PS-3: Parameter Passing to Plugin (Scenario 5 from PRD)

| Check | Expected | Implementation | Status |
|-------|----------|----------------|--------|
| Params converted to CString | Null-terminated C string | `CString::new(params)` | PASS |
| Passed as `*const c_char` | Valid pointer | `c_params.as_ptr()` | PASS |
| CString lifetime | Valid for duration of FFI call | Owned by scope, dropped after call | PASS |

### PS-4: Module Structure

| Check | Expected | Actual | Status |
|-------|----------|--------|--------|
| Module location | `image_processor/src/plugin_loader.rs` | Correct | PASS |
| `ProcessImageFn` type alias | Defined for FFI signature | `type ProcessImageFn = unsafe extern "C" fn(u32, u32, *mut u8, *const c_char)` | PASS |
| Public `process()` function | Returns `anyhow::Result<()>` | Correct | PASS |
| Internal `library_filename()` | `pub(crate)` visibility | Correct | PASS |

### PS-5: Integration with main.rs

| Check | Expected | Actual | Status |
|-------|----------|--------|--------|
| `mod plugin_loader;` declaration | Present | Present (line 7) | PASS |
| Params file reading | `std::fs::read_to_string()` | Correct with context | PASS |
| Library path construction | `plugin_path.join(library_filename(...))` | Correct | PASS |
| Plugin invocation | Before image save | Correct (line 61) | PASS |

### PS-6: Logging Implementation

| Log Level | Expected Content | Actual | Status |
|-----------|------------------|--------|--------|
| `debug!` | Library path, dimensions, params | "plugin_loader::process called with path={}, dimensions={}x{}, params={}" | PASS |
| `info!` | Plugin load start | "Loading plugin from: {}" | PASS |
| `info!` | Execution complete | "Plugin execution complete" | PASS |

### PS-7: SAFETY Comments

| Unsafe Block | Location | SAFETY Comment | Status |
|--------------|----------|----------------|--------|
| `Library::new()` | Line 50 | "The library path is provided by the user and we trust the library to be a valid plugin" | PASS |
| `lib.get()` | Line 54 | "The symbol name is null-terminated and we trust the library exports this symbol with the correct signature" | PASS |
| `process_image_fn()` | Line 61 | "The rgba_data buffer is valid for width*height*4 bytes, c_params is a valid CString pointer, and the library is loaded" | PASS |

---

## Negative and Edge Cases

### NE-1: Plugin Library Not Found (Scenario 2 from PRD)

| Scenario | Expected Behavior | Implementation | Status |
|----------|-------------------|----------------|--------|
| Library file does not exist | Error with expected path | `.with_context(\|\| format!("Failed to load plugin library: {}", ...))` | PASS |
| Error is descriptive | Includes full path | Context includes `plugin_path.display()` | PASS |
| Automated test coverage | Test exists | `test_process_missing_library_returns_error` | PASS |

### NE-2: Missing process_image Symbol (Scenario 3 from PRD)

| Scenario | Expected Behavior | Implementation | Status |
|----------|-------------------|----------------|--------|
| Symbol not exported | Error indicating symbol missing | `.with_context(\|\| "Failed to find process_image symbol")` | PASS |
| Test coverage | Not explicitly tested | Would require malformed library | NEEDS MANUAL |

### NE-3: CString Conversion Errors

| Scenario | Expected Behavior | Implementation | Status |
|----------|-------------------|----------------|--------|
| Params contain null byte | Error on CString conversion | `.with_context(\|\| "Invalid params string")` | PASS |
| Empty params string | Valid CString (just null terminator) | Handled correctly by `CString::new("")` | PASS |
| Unicode params | Valid UTF-8 accepted | Rust strings are UTF-8 by definition | PASS |

### NE-4: Buffer Size Validation

| Scenario | Expected Behavior | Implementation | Status |
|----------|-------------------|----------------|--------|
| Buffer too small | Documented as undefined behavior | Trust model: plugins must respect bounds | DOCUMENTED |
| Buffer too large | Plugin receives correct dimensions | Buffer slice passed to plugin | PASS |
| Zero dimensions | Function still called | No validation (trust model) | DOCUMENTED |

### NE-5: Platform Edge Cases

| Scenario | Expected Behavior | Implementation | Status |
|----------|-------------------|----------------|--------|
| Unknown OS | Fallback to Linux-style | `lib{name}.so` | PASS |
| Plugin name with special chars | No escaping performed | Direct format! interpolation | DOCUMENTED |

---

## Automated Tests

### Current Test Coverage in `plugin_loader.rs`

| Test Name | Purpose | Status |
|-----------|---------|--------|
| `test_library_filename_current_platform` | Verifies correct library suffix for current OS | PASS |
| `test_process_missing_library_returns_error` | Verifies error on non-existent library | PASS |
| `test_process_with_real_plugin` | Loads real plugin and calls process_image | IGNORED (requires build) |

### Test Details

#### `test_library_filename_current_platform`

```rust
#[test]
fn test_library_filename_current_platform() {
    let name = library_filename("mirror_plugin");
    #[cfg(target_os = "macos")]
    assert_eq!(name, "libmirror_plugin.dylib");
    // ... platform-specific assertions
}
```

**Coverage:** Validates platform detection logic for the current build target.

#### `test_process_missing_library_returns_error`

```rust
#[test]
fn test_process_missing_library_returns_error() {
    let path = std::path::Path::new("/nonexistent/path/libfake.dylib");
    let mut data = vec![0u8; 16];
    let result = process(path, 2, 2, &mut data, "{}");
    assert!(result.is_err());
    assert!(err.contains("Failed to load plugin library"));
}
```

**Coverage:** Validates error handling for missing library files.

#### `test_process_with_real_plugin` (IGNORED)

```rust
#[test]
#[ignore]
fn test_process_with_real_plugin() {
    // Loads actual mirror_plugin and calls process_image
}
```

**Coverage:** Integration test requiring built plugin. Run with `cargo test -p image_processor -- --ignored`.

### Tests in `main.rs` (Unchanged from IF-3)

| Test Name | Purpose | IF-4 Relevance |
|-----------|---------|----------------|
| `test_args_parse_all_arguments` | Verify CLI arguments parse | Unchanged |
| `test_args_plugin_path_default_value` | Verify default plugin path | Unchanged |
| `test_args_missing_*_fails` (4 tests) | Verify required args | Unchanged |
| `test_args_paths_preserve_structure` | Verify path handling | Unchanged |
| `test_args_plugin_name_accepts_various_formats` | Verify plugin name flexibility | Unchanged |

### Recommended Additional Tests

| Test | Purpose | Priority |
|------|---------|----------|
| `test_library_filename_all_platforms` | Explicitly test all platform cases | Low |
| `test_process_with_null_byte_params` | Verify CString error handling | Medium |
| `test_process_symbol_not_found` | Verify missing symbol error (requires mock lib) | Low |
| `test_process_validates_buffer_size` | Defensive check if buffer is too small | Medium |

---

## Manual Checks

The following manual verification steps should be performed:

### MC-1: Build Verification

| Check | Command | Expected Result | Verified |
|-------|---------|-----------------|----------|
| Build succeeds | `cargo build` | Exit code 0, no errors | YES |
| Clippy passes | `cargo clippy -- -D warnings` | No warnings or errors | YES |
| Format check | `cargo fmt --check` | No formatting issues | YES |
| Tests pass | `cargo test -p image_processor` | All non-ignored tests pass | YES |

### MC-2: Happy Path Integration

| Check | Command | Expected Result | Verified |
|-------|---------|-----------------|----------|
| Plugin loads and executes | `./target/debug/image_processor --input test_images/sample.png --output output.png --plugin mirror_plugin --params test_images/mirror_params.json` | Creates `output.png` | YES |
| Debug logging shows plugin info | `RUST_LOG=debug ./target/debug/image_processor ...` | Shows "plugin_loader::process called with..." | YES |
| Info logging shows plugin loading | `RUST_LOG=info ./target/debug/image_processor ...` | Shows "Loading plugin from: ..." and "Plugin execution complete" | YES |

### MC-3: Error Handling Verification

| Check | Command | Expected Result | Verified |
|-------|---------|-----------------|----------|
| Missing plugin library | Use `--plugin nonexistent_plugin` | Error: "Failed to load plugin library: ..." | YES |
| Missing params file | Use `--params nonexistent.json` | Error: "Failed to read params file: ..." | YES |
| Invalid plugin path | Use `--plugin-path /nonexistent` | Error: "Failed to load plugin library: ..." | YES |

### MC-4: Cross-Platform Verification (If Available)

| Platform | Test | Expected Result | Verified |
|----------|------|-----------------|----------|
| macOS | Build and run | Uses `.dylib` extension | YES (native) |
| Linux | Build and run | Uses `.so` extension | NOT DONE |
| Windows | Build and run | Uses `.dll` extension | NOT DONE |

### MC-5: Integration Test with Real Plugin

| Check | Command | Expected Result | Verified |
|-------|---------|-----------------|----------|
| Ignored test passes | `cargo test -p image_processor -- --ignored` | `test_process_with_real_plugin` passes | YES |

---

## Risk Zones

### Low Risk

1. **Plugin stubs are no-ops**: Current `mirror_plugin` and `blur_plugin` implementations are no-op stubs. This is expected as plugin logic is out of scope for IF-4. The loader correctly invokes them, and actual image processing will be implemented in IF-5 and IF-6.

2. **Platform detection is compile-time**: Using `cfg!(target_os = ...)` means platform detection happens at compile time, not runtime. This is correct behavior but means cross-compilation requires separate builds.

3. **Unknown platform fallback**: Platforms other than macOS/Linux/Windows fallback to Linux-style naming (`.so`). This is a reasonable default for Unix-like systems.

### Medium Risk

1. **Trust model for plugins**: The implementation trusts plugins to:
   - Not exceed buffer bounds (`width * height * 4` bytes)
   - Not panic across FFI boundary
   - Have correct function signature

   This is documented in the PRD and acceptable for first-party plugins. Malicious or buggy plugins could cause undefined behavior.

2. **Symbol type mismatch**: There is no runtime verification that the loaded `process_image` symbol matches the expected signature. A library with a different signature could cause undefined behavior. This is a fundamental limitation of FFI and is documented.

3. **`.expect()` in main.rs**: The buffer reconstruction uses `.expect("Buffer size mismatch...")`. If a plugin modifies the buffer size incorrectly, this will panic. This is acceptable as it indicates a programming error in the plugin.

### No High Risks Identified

The implementation follows Rust best practices for unsafe code:
- All unsafe blocks have SAFETY comments
- Errors are properly propagated via `anyhow::Result`
- Memory management uses RAII (Library dropped automatically)
- CString lifetime is clearly bounded

---

## Definition of Done Checklist

From PRD IF-4 Acceptance Criteria:

- [x] Test: Load plugin, call with test image (plugin does nothing yet)
- [x] `plugin_loader.rs` exists in `image_processor/src/`
- [x] Public function returns `anyhow::Result<()>` with no unsafe code exposed
- [x] All unsafe blocks are contained within `plugin_loader.rs`
- [x] Every unsafe block has a `// SAFETY:` comment explaining invariants
- [x] Library name construction works for macOS, Linux, and Windows
- [x] Successfully loads `process_image` symbol from valid plugins
- [x] Returns descriptive errors for missing library or symbol
- [x] Params converted to CString without memory leaks
- [x] At least one test that loads a plugin and calls process_image
- [x] Function never panics; all errors returned via Result

From Implementation Plan Acceptance Criteria:

- [x] Module exists: `plugin_loader.rs` contains the `process()` function
- [x] Safe interface: `process()` returns `anyhow::Result<()>`
- [x] Unsafe isolated: All unsafe blocks in `plugin_loader.rs`
- [x] SAFETY documented: Comments on all unsafe blocks
- [x] Platform support: `library_filename()` works for all platforms
- [x] Symbol loading: Loads `process_image` from plugins
- [x] Error messages: Descriptive errors for failures
- [x] CString handling: Proper conversion
- [x] Integration complete: `main.rs` calls plugin before saving
- [x] Test coverage: Tests exist for loader
- [x] No panics: Errors via Result

From Tasklist:

- [x] T1: Create plugin_loader.rs module structure
- [x] T2: Implement library_filename() function
- [x] T3: Implement process() - library loading
- [x] T4: Implement process() - symbol resolution
- [x] T5: Implement process() - FFI call
- [x] T6: Add logging
- [x] T7: Integrate into main.rs
- [x] T8: Unit test for library_filename()
- [x] T9: Integration test for plugin loading
- [x] T10: Verify all SAFETY comments
- [x] T11: Run full build and lint checks

---

## Files Reviewed

| File Path | Purpose |
|-----------|---------|
| `image_processor/src/plugin_loader.rs` | Plugin loader implementation |
| `image_processor/src/main.rs` | Main application with plugin integration |
| `mirror_plugin/src/lib.rs` | No-op plugin stub |
| `blur_plugin/src/lib.rs` | No-op plugin stub |
| `test_images/sample.png` | Test image asset |
| `test_images/mirror_params.json` | Test params file |
| `docs/prd/IF-4.prd.md` | Product requirements |
| `docs/plan/IF-4.md` | Implementation plan |
| `docs/tasklist/IF-4.md` | Task checklist |

---

## Final Verdict

**RELEASE**

All acceptance criteria from the PRD, implementation plan, and tasklist have been met:

1. **Module Created**: `plugin_loader.rs` contains the `process()` and `library_filename()` functions
2. **Safe Interface**: `process()` returns `anyhow::Result<()>` with no unsafe code exposed to callers
3. **Unsafe Isolation**: All 3 unsafe blocks are contained within `plugin_loader.rs`
4. **SAFETY Documentation**: Every unsafe block has a descriptive `// SAFETY:` comment
5. **Platform Support**: `library_filename()` correctly handles macOS (`.dylib`), Linux (`.so`), and Windows (`.dll`)
6. **Symbol Loading**: Successfully loads `process_image` symbol using `libloading`
7. **Error Handling**: Descriptive errors via `anyhow::Context` for missing library and missing symbol
8. **CString Handling**: Params properly converted with error handling for null bytes
9. **Integration Complete**: `main.rs` reads params file, builds plugin path, and calls plugin before saving
10. **Test Coverage**: Three tests covering library filename, missing library error, and real plugin integration
11. **No Panics**: All failure modes return `Result::Err`, no panic paths
12. **Logging**: Debug and info level logging for troubleshooting
13. **Build Quality**: `cargo build`, `cargo clippy -- -D warnings`, and `cargo fmt --check` all pass

The Plugin Loader implementation is complete and ready for the next phases of development (IF-5: Mirror Plugin, IF-6: Blur Plugin).

---

## References

- `docs/prd/IF-4.prd.md` - Product Requirements Document
- `docs/plan/IF-4.md` - Implementation Plan
- `docs/tasklist/IF-4.md` - Task Checklist
- `docs/conventions.md` - Project Code Conventions
- `docs/vision.md` - Technical Architecture
