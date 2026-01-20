# IF-7: Final Polish - Implementation Plan

Status: PLAN_APPROVED

## Overview

This plan covers the final polish iteration for the image processing project. The research phase identified that most tasks (logging, error handling, unit tests) are already substantially complete. The primary remaining work is creating integration tests and README documentation.

## Components to Modify/Create

### 1. Integration Tests (NEW)

**Location:** `image_processor/tests/integration_test.rs`

Creates end-to-end tests that verify the complete workflow: load image, process with plugin, save result.

### 2. README Documentation (NEW)

**Location:** `README.md` (project root)

Comprehensive project documentation with build instructions, usage examples, and test commands.

### 3. Cargo.toml Update (MODIFY)

**Location:** `image_processor/Cargo.toml`

Add dev-dependencies required for integration tests.

### 4. Logging Enhancement (MODIFY - MINOR)

**Location:** `image_processor/src/main.rs`

Add one info-level log message for image loading to match PRD expectations.

---

## API Contract

No new APIs are introduced. This iteration focuses on testing and documentation.

### Existing FFI Contract (Reference)

All plugins export:
```rust
#[unsafe(no_mangle)]
pub unsafe extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
)
```

### Existing CLI Interface (Reference)

```
image_processor --input <PATH> --output <PATH> --plugin <NAME> --params <PATH> [--plugin_path <DIR>]
```

---

## Data Flows

### Integration Test Flow

```
Test Setup
    |
    v
[Create temp directory for output]
    |
    v
[Execute image_processor binary via std::process::Command]
    |
    +-- Input: test_images/sample.png
    +-- Plugin: mirror_plugin or blur_plugin
    +-- Params: test_images/*.json
    +-- Output: temp_dir/output.png
    |
    v
[Verify process exit status]
    |
    v
[Verify output file exists]
    |
    v
[Verify output dimensions match input]
    |
    v
[Temp directory cleanup (automatic)]
```

### Error Test Flow

```
[Execute image_processor with invalid input]
    |
    v
[Verify process fails (non-zero exit)]
    |
    v
[Verify stderr contains meaningful error message]
```

---

## Implementation Tasks

### Task 7.1: Logging Enhancement

**Priority:** Low (optional improvement)
**Effort:** 5 minutes

Add info-level log message before image loading:
```rust
log::info!("Loading image from: {}", args.input.display());
```

**Files Modified:**
- `image_processor/src/main.rs`

### Task 7.2: Error Handling Verification

**Priority:** Low (already complete)
**Effort:** Verification only

Research confirmed:
- All operations use `anyhow::Result` with `.with_context()`
- No bare `.unwrap()` in source code
- Single `.expect()` is documented and acceptable

**Action:** No code changes required.

### Task 7.3: Unit Test Verification

**Priority:** Low (already complete)
**Effort:** Verification only

Research confirmed 39 tests exist:
- 8 tests for CLI parsing (main.rs)
- 4 tests for plugin loader (2 ignored)
- 14 tests for mirror plugin
- 13 tests for blur plugin

**Action:** No code changes required.

### Task 7.4: Integration Tests

**Priority:** High
**Effort:** 1-2 hours

**Files Created:**
- `image_processor/tests/integration_test.rs`

**Files Modified:**
- `image_processor/Cargo.toml` (add dev-dependencies)

**Test Cases:**

1. `test_mirror_plugin_horizontal_flip`
   - Load sample.png, apply mirror plugin, verify output exists and dimensions match

2. `test_blur_plugin_workflow`
   - Load sample.png, apply blur plugin, verify output exists and dimensions match

3. `test_error_nonexistent_input`
   - Verify meaningful error when input file does not exist

4. `test_error_nonexistent_plugin`
   - Verify meaningful error when plugin library does not exist

5. `test_error_invalid_params`
   - Verify meaningful error when params file does not exist

**Dependencies to Add:**
```toml
[dev-dependencies]
tempfile = "3"
image = "0.25"
```

### Task 7.5: README Documentation

**Priority:** High
**Effort:** 30-60 minutes

**Files Created:**
- `README.md`

**Sections:**

1. **Project Description**
   - Purpose: CLI tool for image processing with plugin architecture
   - Architecture: Main binary + dynamically loaded plugins via FFI

2. **Prerequisites**
   - Rust toolchain (edition 2024)
   - Cargo package manager

3. **Build Instructions**
   ```bash
   cargo build                    # Build all (app + plugins)
   cargo build --release          # Build optimized
   ```

4. **Usage Examples**
   - Mirror plugin (horizontal and vertical flip)
   - Blur plugin (configurable radius and iterations)
   - Running with logging enabled

5. **Plugin Parameters**
   - mirror_plugin JSON format
   - blur_plugin JSON format

6. **Test Execution**
   ```bash
   cargo test                     # Run all tests
   cargo test -p image_processor  # Test main app only
   ```

7. **Project Structure**
   - Overview of workspace layout
   - Description of each crate

---

## Non-Functional Requirements

### Testing

| Requirement | Target | Validation |
|-------------|--------|------------|
| All tests pass | `cargo test` exits 0 | CI/manual verification |
| No clippy warnings | `cargo clippy -- -D warnings` exits 0 | CI/manual verification |
| Integration tests execute quickly | < 5 seconds per test | Manual timing |

### Documentation

| Requirement | Target | Validation |
|-------------|--------|------------|
| README completeness | All sections from Task 7.5 present | Manual review |
| Build instructions work | Following README builds project | Manual test |
| Usage examples work | Following README examples produces expected output | Manual test |

### Logging

| Requirement | Target | Validation |
|-------------|--------|------------|
| RUST_LOG=info shows workflow | Loading, processing, saving messages visible | Manual test |
| RUST_LOG=debug shows details | Dimensions, params, paths visible | Manual test |

---

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Integration tests may be slow | Low | Low | Keep test images small, limit iterations |
| Integration tests may be flaky on CI | Medium | Medium | Use tempfile for cleanup, avoid hardcoded paths |
| Platform-specific library names | Low | Medium | Use `cfg` attributes or let libloading handle extensions |
| tempfile dependency not approved | Low | High | Verify tempfile only needed for tests (dev-dependency) |

---

## Dependencies

### New Dependencies

| Crate | Version | Scope | Purpose |
|-------|---------|-------|---------|
| tempfile | 3 | dev-dependency | Temporary directories for test output |
| image | 0.25 | dev-dependency | Verify output image dimensions |

**Note:** `image` is already a runtime dependency, adding it as dev-dependency allows use in integration tests.

### Existing Dependencies (No Changes)

- clap, image, libloading, log, env_logger, anyhow (image_processor)
- log, serde, serde_json (plugins)

---

## Open Questions

None. Requirements are clear from PRD and research findings.

---

## Alternatives Considered

### Integration Test Approach

**Option A: Process-based tests (SELECTED)**
- Run `image_processor` binary via `std::process::Command`
- Pros: Tests real CLI, catches argument parsing issues
- Cons: Requires built binary, slower

**Option B: Library-based tests**
- Call internal functions directly
- Pros: Faster, no binary needed
- Cons: Requires exposing internal API, doesn't test CLI

**Decision:** Option A selected because it provides true end-to-end testing and validates the actual user experience.

### Test File Management

**Option A: tempfile crate (SELECTED)**
- Automatic cleanup via RAII
- Cross-platform temp directory handling

**Option B: Manual temp file creation**
- Use `std::env::temp_dir()` manually
- Requires manual cleanup in test teardown

**Decision:** Option A selected for reliability and cleaner test code.

---

## Implementation Order

1. **Task 7.4: Integration Tests** - Create test infrastructure and test cases
2. **Task 7.5: README.md** - Document build/run/test instructions
3. **Task 7.1: Logging Enhancement** - Add missing info log (optional)
4. **Task 7.2 & 7.3: Verification** - Confirm existing implementation meets requirements

---

## Success Criteria

1. `cargo test` passes all tests (including new integration tests)
2. `cargo clippy -- -D warnings` produces no warnings
3. `RUST_LOG=info ./target/debug/image_processor ...` shows informative log output
4. README.md contains complete build, usage, and test instructions
5. New users can follow README to successfully build and run the application
