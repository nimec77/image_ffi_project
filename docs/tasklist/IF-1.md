# Tasklist: IF-1 - Iteration 1: Project Setup

**Status:** COMPLETED

## Context

Set up the Cargo workspace scaffolding for the Image FFI Project. This iteration creates the foundational project structure with three crates: `image_processor` (main CLI binary), `mirror_plugin` (cdylib), and `blur_plugin` (cdylib). All crates should compile successfully with minimal stub implementations.

---

## Tasks

### Task 1: Create Workspace Root `Cargo.toml`

- [x] Create `/Cargo.toml` at project root with workspace definition
  - [x] Define `[workspace]` section with resolver = "2"
  - [x] List all three members: `image_processor`, `mirror_plugin`, `blur_plugin`

**Acceptance Criteria:**
1. File `/Cargo.toml` exists with `[workspace]` section containing all three members
2. Running `cargo metadata --format-version 1` lists all three crates as workspace members

---

### Task 2: Create `image_processor` Crate

- [x] Create directory `image_processor/src/`
- [x] Create `image_processor/Cargo.toml` with:
  - [x] Package name: `image_processor`, edition 2024
  - [x] Dependencies: clap (features=["derive"]), image, libloading, log, env_logger, anyhow
- [x] Create `image_processor/src/main.rs` with minimal `fn main()` that compiles
- [x] Create `image_processor/src/plugin_loader.rs` as empty stub file

**Acceptance Criteria:**
1. `cargo build -p image_processor` succeeds with exit code 0
2. Binary `target/debug/image_processor` exists and is executable
3. `Cargo.toml` includes all six required dependencies: clap, image, libloading, log, env_logger, anyhow

---

### Task 3: Create `mirror_plugin` Crate

- [x] Create directory `mirror_plugin/src/`
- [x] Create `mirror_plugin/Cargo.toml` with:
  - [x] Package name: `mirror_plugin`, edition 2024
  - [x] `crate-type = ["cdylib"]` in `[lib]` section
  - [x] Dependencies: serde (features=["derive"]), serde_json, log
- [x] Create `mirror_plugin/src/lib.rs` with stub `process_image` FFI function:
  - [x] Function has `#[no_mangle]` attribute
  - [x] Function is `pub extern "C"`
  - [x] Signature: `fn process_image(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char)`
  - [x] Body is empty (no-op)

**Acceptance Criteria:**
1. `cargo build -p mirror_plugin` succeeds with exit code 0
2. Dynamic library exists: `target/debug/libmirror_plugin.dylib` (macOS) or `.so` (Linux) or `.dll` (Windows)
3. `Cargo.toml` specifies `crate-type = ["cdylib"]`

---

### Task 4: Create `blur_plugin` Crate

- [x] Create directory `blur_plugin/src/`
- [x] Create `blur_plugin/Cargo.toml` with:
  - [x] Package name: `blur_plugin`, edition 2024
  - [x] `crate-type = ["cdylib"]` in `[lib]` section
  - [x] Dependencies: serde (features=["derive"]), serde_json, log
- [x] Create `blur_plugin/src/lib.rs` with stub `process_image` FFI function:
  - [x] Function has `#[no_mangle]` attribute
  - [x] Function is `pub extern "C"`
  - [x] Signature: `fn process_image(width: u32, height: u32, rgba_data: *mut u8, params: *const c_char)`
  - [x] Body is empty (no-op)

**Acceptance Criteria:**
1. `cargo build -p blur_plugin` succeeds with exit code 0
2. Dynamic library exists: `target/debug/libblur_plugin.dylib` (macOS) or `.so` (Linux) or `.dll` (Windows)
3. `Cargo.toml` specifies `crate-type = ["cdylib"]`

---

### Task 5: Verify Full Workspace Build

- [x] Run `cargo build` from workspace root
- [x] Verify all three crates compile without errors
- [x] Verify no compilation warnings (or only expected `dead_code` warnings for stubs)
- [x] Verify all expected artifacts exist in `target/debug/`

**Acceptance Criteria:**
1. `cargo build` exits with code 0
2. `target/debug/image_processor` binary exists
3. `target/debug/libmirror_plugin.dylib` (or platform equivalent) exists
4. `target/debug/libblur_plugin.dylib` (or platform equivalent) exists

---

## Verification Commands

```bash
# Build entire workspace
cargo build

# Verify workspace members
cargo metadata --format-version 1 | grep -A 20 '"workspace_members"'

# Check build artifacts exist
ls -la target/debug/image_processor
ls -la target/debug/libmirror_plugin.*
ls -la target/debug/libblur_plugin.*

# Run main binary (should print hello or exit cleanly)
./target/debug/image_processor
```

---

## References

- `docs/prd/IF-1.prd.md` - PRD with detailed requirements
- `docs/plan/IF-1.md` - Implementation plan
- `docs/vision.md` - Technical architecture
- `docs/conventions.md` - Code conventions
