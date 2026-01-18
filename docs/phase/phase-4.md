# Iteration 4: Plugin Loader

**Goal:** Implement the plugin loader module to dynamically load and call plugin libraries via FFI.

## Tasks

- [ ] 4.1 Create `plugin_loader.rs` module
- [ ] 4.2 Implement platform-specific library name (`.dylib`/`.so`/`.dll`)
- [ ] 4.3 Load library with `libloading`
- [ ] 4.4 Get `process_image` symbol
- [ ] 4.5 Call function with SAFETY comment
- [ ] 4.6 Pass params as `CString`

## Acceptance Criteria

**Test:** Load plugin, call with test image (plugin does nothing yet)

## Dependencies

- Phase 3 complete

## Implementation Notes

Reference `docs/vision.md` for the plugin loader design and FFI contract.
