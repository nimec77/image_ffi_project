# IF-3: Image I/O

Status: PRD_READY

## Context / Idea

### Iteration 3: Image I/O

**Goal:** Implement image loading and saving so the application can read PNG files, process them, and write results.

### Tasks from Phase Description

- 3.1 Load PNG with `image` crate
- 3.2 Convert to `RgbaImage`, extract dimensions
- 3.3 Get raw bytes as `Vec<u8>`
- 3.4 Save bytes back to PNG output
- 3.5 Add test image to `test_images/`

### Project Context

This is part of a CLI application for image processing with dynamically loaded plugins via FFI. The main binary loads PNG images, calls plugins to process RGBA data in-place, and saves results.

The application has already implemented:
- Workspace structure with `image_processor` binary crate
- CLI argument parsing using `clap` (IF-2 complete)
- Arguments: `--input`, `--output`, `--plugin`, `--params`, `--plugin-path`

This iteration focuses on the image handling pipeline before plugin integration (which will come in a later iteration).

### Data Flow Reference (from vision.md)

```
1. Parse CLI args (clap)              [DONE - IF-2]
2. Load image -> RgbaImage -> Vec<u8>  [THIS ITERATION]
3. Read params file -> String          [FUTURE]
4. Call plugin_loader::process(...)    [FUTURE]
5. Convert Vec<u8> back -> RgbaImage -> save PNG  [THIS ITERATION]
```

## Goals

1. **Enable PNG file loading**: Read a PNG file from the path specified in `--input` argument and decode it into an RGBA format suitable for processing.

2. **Extract image metadata**: Obtain width and height dimensions from the loaded image.

3. **Provide raw pixel access**: Convert the image to a flat `Vec<u8>` buffer in RGBA format (4 bytes per pixel) that can later be passed to plugins.

4. **Enable PNG file saving**: Convert the processed RGBA buffer back to an image and save it to the path specified in `--output` argument.

5. **Establish test infrastructure**: Add a sample test image to `test_images/` directory for development and testing purposes.

## User Stories

### US-1: Load and Copy Image
**As a** user,
**I want to** run the application with input and output paths,
**So that** the image is loaded and saved (unchanged for now, since no plugin processing is implemented yet).

### US-2: Validate Input File
**As a** user,
**I want to** receive a clear error message when the input file does not exist or is not a valid PNG,
**So that** I understand what went wrong.

### US-3: Handle Output Directory
**As a** user,
**I want to** specify an output path for the processed image,
**So that** the result is saved to my desired location.

### US-4: View Image Dimensions
**As a** developer,
**I want to** see the image dimensions in debug logs,
**So that** I can verify the image was loaded correctly.

## Main Scenarios

### Scenario 1: Successful Image Copy (Happy Path)
**Given** a valid PNG file exists at the input path
**When** the user runs `cargo run -- --input test_images/sample.png --output output.png --plugin mirror_plugin --params params.json`
**Then** the application loads the image, extracts dimensions, gets raw bytes, and saves the unchanged image to the output path

### Scenario 2: Input File Not Found
**Given** the input file does not exist
**When** the user runs the application with a non-existent input path
**Then** the application returns an error with a user-friendly message like "Failed to load image: No such file or directory"

### Scenario 3: Invalid Image Format
**Given** the input file exists but is not a valid PNG or is corrupted
**When** the user attempts to load it
**Then** the application returns an error indicating the image could not be decoded

### Scenario 4: Output Directory Does Not Exist
**Given** the output path specifies a directory that does not exist
**When** the user attempts to save the image
**Then** the application returns an error indicating the output path is invalid

### Scenario 5: Debug Logging
**Given** the environment variable `RUST_LOG=debug` is set
**When** the user runs the application
**Then** the application logs the image dimensions (width x height) and other relevant details

## Success / Metrics

### Acceptance Criteria (from phase-3.md)
- **Test:** `cargo run -- -i test.png -o out.png ...` copies image unchanged

### Functional Metrics
1. Application successfully loads PNG files of various sizes
2. Image dimensions are correctly extracted
3. Raw RGBA buffer has correct size: `width * height * 4` bytes
4. Output PNG is byte-for-byte identical to input (when no processing is applied)
5. Error messages are clear and actionable for common failure cases

### Code Quality Metrics
1. All error handling uses `anyhow::Result` with context
2. No `.unwrap()` calls on file/image operations
3. Logging integrated using `log` crate macros
4. Unit tests cover the image loading and saving logic
5. `cargo clippy -- -D warnings` passes without errors

## Constraints and Assumptions

### Technical Constraints
1. **Image crate**: Must use the `image` crate for PNG loading/saving as specified in project requirements
2. **RGBA format**: Images must be converted to `RgbaImage` (RGBA8) format
3. **Flat buffer**: Pixel data must be accessible as a flat `Vec<u8>` buffer for FFI compatibility
4. **Error handling**: Use `anyhow::Result` exclusively, no custom error types
5. **No unwrap**: Use `?` operator or `.expect("reason")` with clear messages

### Assumptions
1. Input files are PNG format (other formats may work via `image` crate but are not required)
2. The `test_images/` directory exists or will be created
3. CLI arguments are already validated by clap (existence check happens at image load time)
4. Plugin processing will be a no-op for this iteration (pass-through copy)

### Dependencies
- Phase 2 (CLI argument parsing) is complete
- `image` crate is available in Cargo.toml

## Risks

### R1: Large Image Memory Usage
**Risk**: Loading very large images could consume significant memory
**Impact**: Medium
**Mitigation**: Document memory usage expectations; for this iteration, focus on correctness not optimization

### R2: Image Format Compatibility
**Risk**: Some PNG files may have unusual color formats or encoding
**Impact**: Low
**Mitigation**: The `image` crate handles format conversion to RGBA8; test with various PNG types

### R3: File Permission Errors
**Risk**: Output file may not be writable due to permissions
**Impact**: Low
**Mitigation**: Error handling via anyhow will provide clear error messages

### R4: Path Handling Cross-Platform
**Risk**: File path handling differences between Windows/Linux/macOS
**Impact**: Low
**Mitigation**: Use `PathBuf` consistently; `std::path` handles platform differences

## Open Questions

None. The requirements are clear from the project documentation:

1. The `image` crate API for loading, converting, and saving PNG files is well-documented
2. The data flow is clearly specified in `docs/vision.md`
3. The acceptance criteria are defined in `docs/phase/phase-3.md`
4. Error handling approach is defined in `docs/vision.md` (anyhow::Result)
5. Logging approach is defined in `docs/vision.md` (log/env_logger)

The iteration can proceed to implementation planning.
