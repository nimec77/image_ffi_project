# PRD: IF-1 - Iteration 1: Project Setup

**Status:** PRD_READY

## Context / Idea

This is the first iteration of the Image FFI Project - a CLI application for image processing with dynamically loaded plugins via FFI. The primary purpose of this iteration is to establish the foundational project structure using a Cargo workspace.

### Source Reference
- Ticket: IF-1 "Iteration 1: Project Setup"
- Phase: `docs/phase/phase-1.md`

### Background

The Image FFI Project aims to provide practical experience with:
- Working with images in Rust
- Dynamically loading and using libraries
- Interacting via FFI and consciously using unsafe code
- Designing an extensible plugin architecture
- Error handling and safe memory management

This iteration focuses solely on setting up the Cargo workspace scaffolding so that all crates compile successfully before implementing any actual functionality.

### Tasks from Phase-1

1. Create workspace `Cargo.toml` with members
2. Create `image_processor/Cargo.toml` with dependencies
3. Create `mirror_plugin/Cargo.toml` as cdylib
4. Create `blur_plugin/Cargo.toml` as cdylib
5. Create minimal `main.rs` (hello world)
6. Create stub `lib.rs` for each plugin

---

## Goals

1. **Establish Cargo Workspace**: Create a properly configured Cargo workspace that includes all project crates as members.

2. **Configure Main Application Crate**: Set up `image_processor` crate with all required dependencies (`clap`, `image`, `libloading`, `log`, `env_logger`, `anyhow`).

3. **Configure Plugin Crates**: Set up `mirror_plugin` and `blur_plugin` as cdylib crates with appropriate dependencies (`serde`, `serde_json`, `log`).

4. **Create Minimal Compilable Code**: Ensure each crate has minimal valid Rust code that allows `cargo build` to succeed.

5. **Follow Project Conventions**: Adhere to the KISS principle and project structure defined in `docs/vision.md`.

---

## User Stories

### US-1: Developer Sets Up Project
**As a** developer
**I want to** clone the repository and run `cargo build`
**So that** all crates compile without errors and I can start implementing features

**Acceptance Criteria:**
- `cargo build` completes successfully
- All three crates (image_processor, mirror_plugin, blur_plugin) are built
- Plugin crates produce dynamic libraries (`.dylib` on macOS, `.so` on Linux, `.dll` on Windows)

### US-2: Developer Verifies Workspace Structure
**As a** developer
**I want to** see a clear workspace structure
**So that** I understand where to implement each component

**Acceptance Criteria:**
- Workspace `Cargo.toml` lists all three members
- Each crate has its own `Cargo.toml` with appropriate configuration
- Directory structure matches the vision document

---

## Scenarios

### Scenario 1: Fresh Build
**Given** a developer has cloned the repository
**When** they run `cargo build` for the first time
**Then** all crates compile successfully with no errors

### Scenario 2: Plugin Library Output
**Given** the project has been built
**When** the developer checks `target/debug/`
**Then** they find:
- `image_processor` binary (or `image_processor.exe` on Windows)
- `libmirror_plugin.dylib` (macOS) / `libmirror_plugin.so` (Linux) / `mirror_plugin.dll` (Windows)
- `libblur_plugin.dylib` (macOS) / `libblur_plugin.so` (Linux) / `blur_plugin.dll` (Windows)

### Scenario 3: Workspace Members Verification
**Given** the workspace is set up
**When** the developer runs `cargo metadata --format-version 1`
**Then** all three crates are listed as workspace members

---

## Metrics and Success Criteria

| Metric | Target | Measurement Method |
|--------|--------|-------------------|
| Build Success | 100% | `cargo build` exits with code 0 |
| Crates Built | 3 | All members compile (image_processor, mirror_plugin, blur_plugin) |
| Plugin Libraries | 2 | Dynamic libraries exist in target/debug/ |
| Compilation Warnings | 0 (preferred) | `cargo build` output shows no warnings |

### Definition of Done
- [ ] `cargo build` compiles all crates without errors
- [ ] Workspace `Cargo.toml` defines all three members
- [ ] `image_processor/Cargo.toml` includes: clap, image, libloading, log, env_logger, anyhow
- [ ] `mirror_plugin/Cargo.toml` is configured as cdylib with: serde, serde_json, log
- [ ] `blur_plugin/Cargo.toml` is configured as cdylib with: serde, serde_json, log
- [ ] `image_processor/src/main.rs` exists with minimal compilable code
- [ ] `mirror_plugin/src/lib.rs` exists with stub `process_image` function
- [ ] `blur_plugin/src/lib.rs` exists with stub `process_image` function

---

## Constraints

1. **Dependencies**: Only the following crates are permitted:
   - Main app: `clap`, `image`, `libloading`, `log`, `env_logger`, `anyhow`
   - Plugins: `serde`, `serde_json`, `log`

2. **Project Structure**: Must follow the structure defined in `docs/vision.md`:
   ```
   image_ffi_project/
   ├── Cargo.toml                  # Workspace definition
   ├── image_processor/
   │   ├── Cargo.toml
   │   └── src/
   │       ├── main.rs
   │       └── plugin_loader.rs    # (can be empty/stub for this iteration)
   ├── mirror_plugin/
   │   ├── Cargo.toml              # crate-type = ["cdylib"]
   │   └── src/lib.rs
   └── blur_plugin/
       ├── Cargo.toml              # crate-type = ["cdylib"]
       └── src/lib.rs
   ```

3. **Plugin Crate Type**: Plugins must be compiled as `cdylib` for dynamic loading.

4. **No Functionality Required**: This iteration only requires scaffolding - no actual image processing logic needed yet.

---

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Dependency version conflicts | Low | Medium | Use compatible versions from crates.io, test build early |
| Platform-specific build issues | Low | Medium | Test on target platform; document any platform-specific requirements |
| Incorrect cdylib configuration | Low | High | Verify plugin libraries are generated in target/debug/ after build |

---

## Open Questions

None - all requirements are clearly defined in the phase documentation and vision document. This iteration is straightforward scaffolding work with no ambiguity.

---

## References

- `/Users/comrade77/RustroverProjects/image_ffi_project/docs/idea.md` - Full project requirements
- `/Users/comrade77/RustroverProjects/image_ffi_project/docs/vision.md` - Technical architecture and design
- `/Users/comrade77/RustroverProjects/image_ffi_project/docs/conventions.md` - Code conventions
- `/Users/comrade77/RustroverProjects/image_ffi_project/docs/phase/phase-1.md` - Iteration 1 task list
