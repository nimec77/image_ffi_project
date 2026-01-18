# Tasklist: IF-2 - CLI Arguments

**Status:** TASKLIST_READY

## Context

Implement command-line argument parsing using clap derive macros. This replaces the stub `main.rs` with a proper CLI interface accepting 5 arguments: input, output, plugin, params, and plugin-path.

---

## Tasks

- [ ] **Task 1: Add required imports**
  - Add `use anyhow::Result;`, `use clap::Parser;`, and `use std::path::PathBuf;` to `image_processor/src/main.rs`
  - **Acceptance Criteria:**
    - All three imports are present at the top of the file
    - No unused import warnings

- [ ] **Task 2: Define Args struct with clap derive**
  - Create `Args` struct with `#[derive(Parser)]` attribute
  - Add 5 fields: `input` (PathBuf), `output` (PathBuf), `plugin` (String), `params` (PathBuf), `plugin_path` (PathBuf)
  - Add `#[arg(long)]` attribute to all fields
  - Add `#[arg(default_value = "target/debug")]` to `plugin_path`
  - Add doc comments for each field (these become help text)
  - **Acceptance Criteria:**
    - Struct compiles without errors
    - `cargo run -- --help` shows all 5 arguments with descriptions

- [ ] **Task 3: Update main function**
  - Change signature to `fn main() -> Result<()>`
  - Call `Args::parse()` to parse command-line arguments
  - Print each parsed argument for verification
  - Return `Ok(())`
  - **Acceptance Criteria:**
    - `cargo build` succeeds without errors or warnings
    - Running with valid arguments prints all parsed values

- [ ] **Task 4: Verify CLI functionality**
  - Run `cargo run -- --help` and verify usage output
  - Run with all arguments and verify parsing works
  - Run without `--plugin-path` and verify default is `target/debug`
  - Run without required argument and verify helpful error message
  - **Acceptance Criteria:**
    - Help output shows all 5 arguments with descriptions
    - Default value for `--plugin-path` is displayed and works correctly
    - Missing required arguments produce clear error messages
