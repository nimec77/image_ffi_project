# Iteration 4: Plugin Loader

**Goal:** Implement the plugin loader module to dynamically load and call plugin libraries via FFI.

## Tasks

- [x] 4.1 Create `plugin_loader.rs` module
- [x] 4.2 Implement platform-specific library name (`.dylib`/`.so`/`.dll`)
- [x] 4.3 Load library with `libloading`
- [x] 4.4 Get `process_image` symbol
- [x] 4.5 Call function with SAFETY comment
- [x] 4.6 Pass params as `CString`

## Acceptance Criteria

**Test:** Load plugin, call with test image (plugin does nothing yet)

## Dependencies

- Phase 3 complete

## Implementation Notes

Reference `docs/vision.md` for the plugin loader design and FFI contract.
