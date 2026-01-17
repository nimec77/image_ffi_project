# Iteration 1: Project Setup

**Goal:** Set up the Cargo workspace with all crate scaffolding so `cargo build` compiles successfully.

## Tasks

- [ ] 1.1 Create workspace `Cargo.toml` with members
- [ ] 1.2 Create `image_processor/Cargo.toml` with dependencies
- [ ] 1.3 Create `mirror_plugin/Cargo.toml` as cdylib
- [ ] 1.4 Create `blur_plugin/Cargo.toml` as cdylib
- [ ] 1.5 Create minimal `main.rs` (hello world)
- [ ] 1.6 Create stub `lib.rs` for each plugin

## Acceptance Criteria

**Test:** `cargo build` compiles all crates

## Dependencies

- None (first iteration)

## Implementation Notes

Reference `docs/vision.md` for workspace structure and dependency versions.
