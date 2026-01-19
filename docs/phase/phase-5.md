# Iteration 5: Mirror Plugin

**Goal:** Implement the mirror plugin that flips images horizontally and/or vertically based on JSON parameters.

## Tasks

- [x] 5.1 Define `Params` struct (horizontal, vertical)
- [x] 5.2 Parse JSON params with serde
- [x] 5.3 Implement horizontal flip
- [x] 5.4 Implement vertical flip
- [x] 5.5 Add `test_images/mirror_params.json`

## Acceptance Criteria

**Test:** `cargo run -- ... --plugin mirror_plugin` flips image correctly

## Dependencies

- Phase 4 complete

## Implementation Notes

Reference `docs/vision.md` for plugin FFI contract and parameter structure.
