# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **IF-2: CLI Arguments** - Implemented command-line argument parsing using clap derive macros
  - `Args` struct with 5 fields: `input`, `output`, `plugin`, `params`, `plugin_path`
  - Long flags for all arguments (`--input`, `--output`, `--plugin`, `--params`, `--plugin-path`)
  - Default value `target/debug` for `--plugin-path` argument
  - Auto-generated help documentation via `--help` flag
  - 8 unit tests covering argument parsing, default values, and error cases

- **IF-1: Project Scaffolding** - Established Cargo workspace with three crates
  - Workspace `Cargo.toml` with resolver version 2
  - `image_processor` binary crate with dependencies: clap, image, libloading, log, env_logger, anyhow
  - `mirror_plugin` cdylib crate with dependencies: serde, serde_json, log
  - `blur_plugin` cdylib crate with dependencies: serde, serde_json, log
  - FFI stub functions (`process_image`) in both plugins with correct signature
  - Minimal main.rs entry point and plugin_loader.rs stub file
