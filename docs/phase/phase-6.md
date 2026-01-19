# Iteration 6: Blur Plugin

**Goal:** Implement the blur plugin that applies a weighted average blur algorithm to images based on JSON parameters.

## Tasks

- [ ] 6.1 Define `Params` struct (radius, iterations)
- [ ] 6.2 Parse JSON params with serde
- [ ] 6.3 Implement weighted average blur algorithm
- [ ] 6.4 Support multiple iterations
- [ ] 6.5 Add `test_images/blur_params.json`

## Acceptance Criteria

**Test:** `cargo run -- ... --plugin blur_plugin` blurs image correctly

## Dependencies

- Phase 5 complete

## Implementation Notes

Reference `docs/vision.md` for plugin FFI contract and parameter structure.
