# IF-4: Plugin Loader

Status: PRD_READY

## Context / Idea

**Goal:** Implement the plugin loader module to dynamically load and call plugin libraries via FFI.

### Tasks

- 4.1 Create `plugin_loader.rs` module
- 4.2 Implement platform-specific library name (`.dylib`/`.so`/`.dll`)
- 4.3 Load library with `libloading`
- 4.4 Get `process_image` symbol
- 4.5 Call function with SAFETY comment
- 4.6 Pass params as `CString`

### Acceptance Criteria

**Test:** Load plugin, call with test image (plugin does nothing yet)

### Dependencies

- Phase 3 (Image I/O) complete

### Implementation Notes

Reference `docs/vision.md` for the plugin loader design and FFI contract.

## Goals

1. **Create plugin loader module** - Implement `image_processor/src/plugin_loader.rs` to isolate all unsafe FFI code from the rest of the application.

2. **Platform-agnostic library loading** - Support loading dynamic libraries across macOS (`.dylib`), Linux (`.so`), and Windows (`.dll`) by constructing the correct library filename based on the operating system.

3. **Safe FFI interface** - Provide a safe Rust function that internally handles all unsafe operations with proper SAFETY documentation.

4. **Symbol resolution** - Load the `process_image` symbol from plugin libraries matching the defined FFI contract.

5. **Parameter marshalling** - Convert Rust strings to C strings (`CString`) for passing parameters to plugins.

## User Stories

### US-1: Developer integrates plugin loader

As a developer working on the image_processor,
I want a `plugin_loader::process()` function that handles all FFI complexity,
So that `main.rs` can call plugins without touching raw pointers or unsafe code.

### US-2: Cross-platform plugin loading

As a user running the application on different operating systems,
I want the application to automatically find the correct plugin library file,
So that I can use the same command regardless of whether I am on macOS, Linux, or Windows.

### US-3: Plugin execution with parameters

As a user applying an image filter,
I want to pass a JSON parameter file to the plugin,
So that I can customize the plugin behavior (e.g., mirror direction, blur radius).

### US-4: Error feedback on plugin issues

As a user,
I want clear error messages when a plugin cannot be loaded or is missing,
So that I can troubleshoot issues without needing to understand FFI internals.

## Main Scenarios

### Scenario 1: Successful plugin loading and execution (Positive)

**Given:** A valid plugin library exists at the specified path (e.g., `target/debug/libmirror_plugin.dylib`)
**And:** The library exports the `process_image` function with the correct signature
**When:** The user calls the image_processor with the plugin name
**Then:** The plugin loader:
1. Constructs the platform-specific library filename
2. Loads the library using `libloading`
3. Retrieves the `process_image` symbol
4. Converts the params string to CString
5. Calls the function with image data (width, height, rgba_data pointer, params pointer)
6. Returns successfully after the plugin completes

### Scenario 2: Plugin library not found (Negative)

**Given:** The specified plugin library does not exist in the plugin path
**When:** The user calls the image_processor with that plugin name
**Then:** The plugin loader returns an error with a message indicating the library file was not found, including the expected path.

### Scenario 3: Missing process_image symbol (Negative)

**Given:** A library file exists but does not export the `process_image` function
**When:** The user attempts to load this library as a plugin
**Then:** The plugin loader returns an error indicating the symbol could not be found.

### Scenario 4: Cross-platform library naming (Positive)

**Given:** A plugin named "mirror_plugin"
**When:** The plugin loader constructs the library path
**Then:**
- On macOS: `{plugin_path}/libmirror_plugin.dylib`
- On Linux: `{plugin_path}/libmirror_plugin.so`
- On Windows: `{plugin_path}/mirror_plugin.dll`

### Scenario 5: Parameter passing to plugin (Positive)

**Given:** A params string containing JSON: `{"horizontal": true, "vertical": false}`
**When:** The plugin loader calls the process_image function
**Then:** The params string is converted to a null-terminated CString and passed as a `*const c_char` pointer.

## Success / Metrics

| Metric | Success Criteria |
|--------|------------------|
| Module creation | `plugin_loader.rs` exists in `image_processor/src/` |
| Safe interface | Public function returns `anyhow::Result<()>` with no unsafe code exposed |
| Unsafe isolation | All unsafe blocks are contained within `plugin_loader.rs` |
| SAFETY comments | Every unsafe block has a `// SAFETY:` comment explaining invariants |
| Platform support | Library name construction works for macOS, Linux, and Windows |
| Symbol loading | Successfully loads `process_image` symbol from valid plugins |
| Error handling | Returns descriptive errors for missing library or symbol |
| CString handling | Params converted to CString without memory leaks |
| Test coverage | At least one test that loads a plugin and calls process_image |
| No panics | Function never panics; all errors returned via Result |

## Constraints and Assumptions

### Constraints

1. **Dependencies** - Only `libloading` crate for dynamic loading (no alternatives).
2. **No custom errors** - Use `anyhow::Result` and `anyhow::bail!` or `anyhow::Context` for errors.
3. **Unsafe isolation** - All unsafe code must be in `plugin_loader.rs`, not in `main.rs`.
4. **FFI contract** - Plugins must match the exact signature:
   ```rust
   extern "C" fn process_image(
       width: u32,
       height: u32,
       rgba_data: *mut u8,
       params: *const c_char,
   )
   ```
5. **No bare unwrap** - Use `?` or `.expect("reason")` with clear messages.

### Assumptions

1. Plugin libraries are built as `cdylib` and located in the specified plugin path.
2. The `process_image` function exists in all valid plugins with the expected signature.
3. The RGBA data buffer is valid and has exactly `width * height * 4` bytes.
4. Plugins will not exceed buffer bounds or cause undefined behavior.
5. The library can be safely dropped after the function call completes.

## Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Plugin crashes or panics | Application crash, undefined behavior | Document that plugins must not panic across FFI; add logging before/after calls |
| Memory corruption from malformed plugin | Data corruption, security risk | Trust plugins as first-party code; future work could add validation |
| Library not unloaded properly | Resource leaks | Use `libloading::Library` with RAII (dropped automatically) |
| CString conversion fails on invalid UTF-8 | Error during parameter passing | Params file should be valid UTF-8; CString::new returns Result |
| Platform detection fails | Wrong library extension | Use `cfg!(target_os = "...")` which is compile-time checked |
| Symbol type mismatch | Undefined behavior at runtime | Document exact signature; no runtime validation possible |

## Open Questions

None - the requirements from `docs/vision.md` and `docs/idea.md` provide sufficient detail for implementation. The FFI contract, function signature, platform-specific naming, and error handling approaches are all clearly defined.
