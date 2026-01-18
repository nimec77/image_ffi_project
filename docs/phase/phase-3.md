# Iteration 3: Image I/O

**Goal:** Implement image loading and saving so the application can read PNG files, process them, and write results.

## Tasks

- [ ] 3.1 Load PNG with `image` crate
- [ ] 3.2 Convert to `RgbaImage`, extract dimensions
- [ ] 3.3 Get raw bytes as `Vec<u8>`
- [ ] 3.4 Save bytes back to PNG output
- [ ] 3.5 Add test image to `test_images/`

## Acceptance Criteria

**Test:** `cargo run -- -i test.png -o out.png ...` copies image unchanged

## Dependencies

- Phase 2 complete

## Implementation Notes

Reference `docs/vision.md` for image handling patterns and the `image` crate API.
