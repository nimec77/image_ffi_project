# IF-5: Mirror Plugin

Status: PRD_READY

## Context / Idea

**Iteration 5: Mirror Plugin**

This phase implements the mirror plugin that flips images horizontally and/or vertically based on JSON parameters. The plugin is part of the Image FFI Project, a CLI application for image processing with dynamically loaded plugins via FFI.

The mirror plugin is one of two required plugins (alongside the blur plugin) specified in the project requirements. It receives RGBA image data through the FFI boundary and modifies it in-place to flip the image along the horizontal and/or vertical axis.

### Current State

- The `mirror_plugin` crate exists with the correct cdylib configuration
- A stub `process_image` FFI function is in place (no-op implementation)
- Dependencies are already configured: `serde`, `serde_json`, `log`
- The plugin loader (Phase 4) is complete and can load plugins dynamically

### Phase 5 Tasks (from `docs/phase/phase-5.md`)

- 5.1 Define `Params` struct (horizontal, vertical)
- 5.2 Parse JSON params with serde
- 5.3 Implement horizontal flip
- 5.4 Implement vertical flip
- 5.5 Add `test_images/mirror_params.json`

## Goals

1. **Implement horizontal flip**: Swap pixels from left to right within each row
2. **Implement vertical flip**: Swap rows from top to bottom
3. **Support combined operations**: Allow both horizontal and vertical flips in a single call
4. **Parse JSON parameters**: Accept `{"horizontal": bool, "vertical": bool}` configuration
5. **Maintain FFI safety**: Follow all plugin safety rules for in-place modification

## User Stories

### US-1: Horizontal Image Flip
As a user, I want to flip an image horizontally (mirror effect) so that the left side becomes the right side and vice versa.

### US-2: Vertical Image Flip
As a user, I want to flip an image vertically so that the top becomes the bottom and vice versa.

### US-3: Combined Flip
As a user, I want to apply both horizontal and vertical flips simultaneously to achieve a 180-degree rotation effect.

### US-4: No-Op with Default Parameters
As a user, when I provide parameters with both `horizontal` and `vertical` set to `false`, I expect the image to remain unchanged.

## Main Scenarios

### Scenario 1: Horizontal Flip Only
```bash
# mirror_params.json: {"horizontal": true, "vertical": false}
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params test_images/mirror_params.json
```
**Expected**: Output image is a horizontal mirror of the input.

### Scenario 2: Vertical Flip Only
```bash
# mirror_params.json: {"horizontal": false, "vertical": true}
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params test_images/mirror_params.json
```
**Expected**: Output image is vertically flipped (upside-down).

### Scenario 3: Both Flips (180-degree rotation)
```bash
# mirror_params.json: {"horizontal": true, "vertical": true}
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params test_images/mirror_params.json
```
**Expected**: Output image is rotated 180 degrees.

### Scenario 4: Invalid JSON Parameters
```bash
# mirror_params.json: {"invalid": "json"}
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin mirror_plugin \
    --params test_images/mirror_params.json
```
**Expected**: Plugin logs an error and returns early without modifying the image.

### Scenario 5: Empty Parameters
```bash
# mirror_params.json: {}
```
**Expected**: Plugin uses default values (both false) or handles gracefully.

## Success / Metrics

### Functional Criteria
- [ ] Horizontal flip correctly swaps pixels within each row
- [ ] Vertical flip correctly swaps rows
- [ ] Combined flip produces correct 180-degree rotation
- [ ] No-op when both parameters are false
- [ ] Invalid JSON parameters result in early return with error log

### Technical Criteria
- [ ] All unsafe blocks have `// SAFETY:` comments
- [ ] No memory access beyond `width * height * 4` bytes
- [ ] No panics across FFI boundary
- [ ] Unit tests pass for core flip logic
- [ ] Integration test: `cargo run -- ... --plugin mirror_plugin` works correctly

### Code Quality Criteria
- [ ] `cargo build` succeeds without warnings
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] `cargo test -p mirror_plugin` passes

## Constraints and Assumptions

### Constraints
1. **In-place modification only**: Cannot allocate new buffers; must swap pixels in the existing RGBA buffer
2. **Allowed dependencies**: Only `serde`, `serde_json`, and `log` (already configured)
3. **FFI contract**: Must export `process_image` with exact signature from `docs/vision.md`
4. **KISS principle**: Simple, straightforward implementation without over-engineering

### Assumptions
1. Input image is always valid RGBA8 format with 4 bytes per pixel
2. Buffer size is always exactly `width * height * 4` bytes
3. Parameters file contains valid UTF-8 text (may or may not be valid JSON)
4. Plugin loader (Phase 4) correctly passes parameters as null-terminated C string

### Technical Details
- **Pixel format**: RGBA with 4 bytes per pixel (R, G, B, A order)
- **Buffer layout**: Row-major order, starting from top-left corner
- **Pixel access**: `pixel_index = (y * width + x) * 4`

## Risks

### Low Risk
1. **Algorithm complexity**: Flip algorithms are well-understood and straightforward
2. **Dependency issues**: All dependencies are already configured and tested

### Medium Risk
1. **Edge cases in pixel swapping**: Off-by-one errors in loop bounds for odd dimensions
   - *Mitigation*: Careful testing with various image sizes including odd dimensions
2. **In-place swap complexity**: Swapping pixels in-place requires temporary storage or careful ordering
   - *Mitigation*: Use standard swap patterns; test thoroughly

### Considerations
1. **Performance**: For very large images, the flip operation should still be efficient
   - *Mitigation*: Use simple O(n) algorithms with minimal memory overhead

## Open Questions

None. The requirements are clear from the project documentation:
- Parameter format is defined in `docs/vision.md`
- FFI contract is specified
- Plugin structure is established
- All dependencies are already in place

The implementation can proceed based on the existing specifications.
