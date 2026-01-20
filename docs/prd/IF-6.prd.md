# IF-6: Blur Plugin

Status: PRD_READY

## Context / Idea

**Iteration 6: Blur Plugin**

This phase implements the blur plugin that applies a weighted average blur algorithm to images based on JSON parameters. The plugin is part of the Image FFI Project, a CLI application for image processing with dynamically loaded plugins via FFI.

The blur plugin is one of two required plugins (alongside the mirror plugin) specified in the project requirements. It receives RGBA image data through the FFI boundary and modifies it in-place to apply a blur effect using a weighted average algorithm where pixel weights are based on distance from the center.

### Algorithm Description (from docs/idea.md)

The new pixel color is equal to the weighted average of the colors of the pixels within the blur radius, where the weight is the distance between the center pixel and the current pixel.

### Current State

- The `blur_plugin` crate exists with the correct cdylib configuration
- A stub `process_image` FFI function is in place (no-op implementation)
- Dependencies are already configured: `serde`, `serde_json`, `log`
- The plugin loader (Phase 4) is complete and can load plugins dynamically
- The mirror plugin (Phase 5) is complete and provides a reference implementation pattern

### Phase 6 Tasks (from docs/phase/phase-6.md)

- 6.1 Define `Params` struct (radius, iterations)
- 6.2 Parse JSON params with serde
- 6.3 Implement weighted average blur algorithm
- 6.4 Support multiple iterations
- 6.5 Add `test_images/blur_params.json`

## Goals

1. **Implement weighted average blur**: Apply blur effect where each pixel becomes a weighted average of its neighbors within a given radius
2. **Support configurable radius**: Accept a `radius` parameter that determines the blur kernel size
3. **Support multiple iterations**: Accept an `iterations` parameter to apply the blur effect multiple times for stronger blurring
4. **Parse JSON parameters**: Accept `{"radius": u32, "iterations": u32}` configuration
5. **Maintain FFI safety**: Follow all plugin safety rules for in-place modification

## User Stories

### US-1: Basic Blur Effect
As a user, I want to apply a blur effect to an image so that it appears softer and less sharp.

### US-2: Configurable Blur Strength via Radius
As a user, I want to specify the blur radius so that I can control how much area around each pixel is considered for blurring.

### US-3: Configurable Blur Intensity via Iterations
As a user, I want to specify the number of blur iterations so that I can achieve stronger blur effects by applying the algorithm multiple times.

### US-4: Minimal Parameters
As a user, when I provide minimal parameters (e.g., radius=1, iterations=1), I expect a subtle blur effect.

## Main Scenarios

### Scenario 1: Basic Blur
```bash
# blur_params.json: {"radius": 3, "iterations": 1}
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin blur_plugin \
    --params test_images/blur_params.json
```
**Expected**: Output image has a subtle blur effect with radius 3.

### Scenario 2: Strong Blur (Multiple Iterations)
```bash
# blur_params.json: {"radius": 5, "iterations": 3}
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin blur_plugin \
    --params test_images/blur_params.json
```
**Expected**: Output image has a strong blur effect from 3 iterations of radius-5 blur.

### Scenario 3: Minimal Blur
```bash
# blur_params.json: {"radius": 1, "iterations": 1}
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin blur_plugin \
    --params test_images/blur_params.json
```
**Expected**: Output image has a very subtle blur, averaging only immediate neighbors.

### Scenario 4: Invalid JSON Parameters
```bash
# blur_params.json: {"invalid": "json"}
./target/debug/image_processor \
    --input test_images/sample.png \
    --output output.png \
    --plugin blur_plugin \
    --params test_images/blur_params.json
```
**Expected**: Plugin logs an error and returns early without modifying the image.

### Scenario 5: Zero Radius or Zero Iterations
```bash
# blur_params.json: {"radius": 0, "iterations": 1}
# or: {"radius": 5, "iterations": 0}
```
**Expected**: Plugin handles gracefully - either no-op or uses sensible defaults.

## Success / Metrics

### Functional Criteria
- [ ] Blur effect correctly averages pixels within the specified radius
- [ ] Weight calculation uses distance from center pixel
- [ ] Multiple iterations produce progressively stronger blur
- [ ] Radius of 0 or iterations of 0 results in no change or sensible handling
- [ ] Invalid JSON parameters result in early return with error log

### Technical Criteria
- [ ] All unsafe blocks have `// SAFETY:` comments
- [ ] No memory access beyond `width * height * 4` bytes
- [ ] No panics across FFI boundary
- [ ] Unit tests pass for core blur logic
- [ ] Integration test: `cargo run -- ... --plugin blur_plugin` works correctly

### Code Quality Criteria
- [ ] `cargo build` succeeds without warnings
- [ ] `cargo clippy -- -D warnings` passes
- [ ] `cargo fmt --check` passes
- [ ] `cargo test -p blur_plugin` passes

## Constraints and Assumptions

### Constraints
1. **In-place modification**: Must work within the existing RGBA buffer; may require temporary buffer for intermediate results during blur calculation
2. **Allowed dependencies**: Only `serde`, `serde_json`, and `log` (already configured)
3. **FFI contract**: Must export `process_image` with exact signature from `docs/vision.md`
4. **KISS principle**: Simple, straightforward implementation without over-engineering
5. **No additional crates**: Cannot use external image processing libraries

### Assumptions
1. Input image is always valid RGBA8 format with 4 bytes per pixel
2. Buffer size is always exactly `width * height * 4` bytes
3. Parameters file contains valid UTF-8 text (may or may not be valid JSON)
4. Plugin loader correctly passes parameters as null-terminated C string

### Technical Details
- **Pixel format**: RGBA with 4 bytes per pixel (R, G, B, A order)
- **Buffer layout**: Row-major order, starting from top-left corner
- **Pixel access**: `pixel_index = (y * width + x) * 4`
- **Weight formula**: Weight is inversely related to distance from center pixel
- **Edge handling**: Pixels at image edges should only average available neighbors (no wrap-around)

### Parameter Structure (from docs/vision.md)
```json
{"radius": 5, "iterations": 2}
```

## Risks

### Low Risk
1. **Algorithm clarity**: Weighted average blur is a standard algorithm with clear mathematical definition
2. **Dependency issues**: All dependencies are already configured and tested
3. **Reference implementation**: Mirror plugin provides a working pattern for FFI, parameter parsing, and testing

### Medium Risk
1. **Performance for large images/radii**: Blur is O(n * r^2) where n is pixel count and r is radius
   - *Mitigation*: Accept this as inherent to the algorithm; document expected performance
2. **Memory for intermediate buffer**: May need temporary buffer to avoid reading modified pixels
   - *Mitigation*: Allocate temporary buffer within plugin; this is acceptable as it's freed before returning
3. **Edge case handling**: Pixels at image boundaries have fewer neighbors
   - *Mitigation*: Carefully handle edge cases; only average available pixels within bounds

### Considerations
1. **Numerical precision**: Floating-point arithmetic for weight calculations and averaging
   - *Mitigation*: Use f64 for intermediate calculations, round to u8 for final values
2. **Alpha channel handling**: Should alpha be blurred or preserved?
   - *Assumption*: Blur all four channels (R, G, B, A) uniformly for simplicity

## Open Questions

None. The requirements are clear from the project documentation:
- Parameter format is defined in `docs/vision.md` (`{"radius": u32, "iterations": u32}`)
- Algorithm is specified in `docs/idea.md` (weighted average based on distance)
- FFI contract is specified in `docs/vision.md`
- Plugin structure is established (mirror_plugin provides reference)
- All dependencies are already in place

The implementation can proceed based on the existing specifications.
