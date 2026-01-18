# Development Plan

> Reference: [idea.md](idea.md) | [vision.md](vision.md)

## Progress Report

| # | Iteration | Status | Tested |
|:-:|-----------|:------:|:------:|
| 1 | Project Setup | ✅ | ✅ |
| 2 | CLI Arguments | ✅ | ✅ |
| 3 | Image I/O | ✅ | ✅ |
| 4 | Plugin Loader | ✅ | ✅ |
| 5 | Mirror Plugin | ⬜ | ⬜ |
| 6 | Blur Plugin | ⬜ | ⬜ |
| 7 | Final Polish | ⬜ | ⬜ |

**Legend**: ⬜ Pending | 🔄 In Progress | ✅ Complete | ❌ Blocked

**Current Phase:** 5

---

## Iteration 1: Project Setup

- [x] Create workspace `Cargo.toml` with members
- [x] Create `image_processor/Cargo.toml` with dependencies
- [x] Create `mirror_plugin/Cargo.toml` as cdylib
- [x] Create `blur_plugin/Cargo.toml` as cdylib
- [x] Create minimal `main.rs` (hello world)
- [x] Create stub `lib.rs` for each plugin

**Test**: `cargo build` compiles all crates ✅

---

## Iteration 2: CLI Arguments

- [x] Define `Args` struct with clap derive
- [x] Add all 5 arguments (input, output, plugin, params, plugin-path)
- [x] Print parsed arguments to verify

**Test**: `cargo run -- --help` shows usage ✅

---

## Iteration 3: Image I/O

- [x] Load PNG with `image` crate
- [x] Convert to `RgbaImage`, extract dimensions
- [x] Get raw bytes as `Vec<u8>`
- [x] Save bytes back to PNG output
- [x] Add test image to `test_images/`

**Test**: `cargo run -- -i test.png -o out.png ...` copies image unchanged ✅

---

## Iteration 4: Plugin Loader

- [x] Create `plugin_loader.rs` module
- [x] Implement platform-specific library name (`.dylib`/`.so`/`.dll`)
- [x] Load library with `libloading`
- [x] Get `process_image` symbol
- [x] Call function with SAFETY comment
- [x] Pass params as `CString`

**Test**: Load plugin, call with test image (plugin does nothing yet) ✅

---

## Iteration 5: Mirror Plugin

- [ ] Define `Params` struct (horizontal, vertical)
- [ ] Parse JSON params with serde
- [ ] Implement horizontal flip
- [ ] Implement vertical flip
- [ ] Add `test_images/mirror_params.json`

**Test**: `cargo run -- ... --plugin mirror_plugin` flips image correctly

---

## Iteration 6: Blur Plugin

- [ ] Define `Params` struct (radius, iterations)
- [ ] Parse JSON params with serde
- [ ] Implement weighted average blur algorithm
- [ ] Support multiple iterations
- [ ] Add `test_images/blur_params.json`

**Test**: `cargo run -- ... --plugin blur_plugin` blurs image correctly

---

## Iteration 7: Final Polish

- [ ] Add logging (`env_logger::init()`, log macros)
- [ ] Verify all error paths return `anyhow::Result`
- [ ] Add unit tests for plugin logic
- [ ] Add integration test for full workflow
- [ ] Update README.md

**Test**: `cargo test` passes, `RUST_LOG=info` shows logs
