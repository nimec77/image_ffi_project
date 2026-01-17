# Iteration 1: Project Setup

**Goal:** Set up the Cargo workspace with all crate scaffolding so `cargo build` compiles successfully.

## Tasks

- [x] 1.1 Create workspace `Cargo.toml` with members
- [x] 1.2 Create `image_processor/Cargo.toml` with dependencies
- [x] 1.3 Create `mirror_plugin/Cargo.toml` as cdylib
- [x] 1.4 Create `blur_plugin/Cargo.toml` as cdylib
- [x] 1.5 Create minimal `main.rs` (hello world)
- [x] 1.6 Create stub `lib.rs` for each plugin

## Acceptance Criteria

**Test:** `cargo build` compiles all crates

## Dependencies

- None (first iteration)

## Implementation Notes

Reference `docs/vision.md` for workspace structure and dependency versions.
