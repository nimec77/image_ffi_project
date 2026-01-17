# Development Plan

> Reference: [idea.md](idea.md) | [vision.md](vision.md)

## Progress Report

| # | Iteration | Status | Tested |
|:-:|-----------|:------:|:------:|
| 1 | Project Setup | ⬜ | ⬜ |
| 2 | CLI Arguments | ⬜ | ⬜ |
| 3 | Image I/O | ⬜ | ⬜ |
| 4 | Plugin Loader | ⬜ | ⬜ |
| 5 | Mirror Plugin | ⬜ | ⬜ |
| 6 | Blur Plugin | ⬜ | ⬜ |
| 7 | Final Polish | ⬜ | ⬜ |

**Legend**: ⬜ Pending | 🔄 In Progress | ✅ Complete | ❌ Blocked

**Current Phase:** 1

---

## Iteration 1: Project Setup

- [ ] Create workspace `Cargo.toml` with members
- [ ] Create `image_processor/Cargo.toml` with dependencies
- [ ] Create `mirror_plugin/Cargo.toml` as cdylib
- [ ] Create `blur_plugin/Cargo.toml` as cdylib
- [ ] Create minimal `main.rs` (hello world)
- [ ] Create stub `lib.rs` for each plugin

**Test**: `cargo build` compiles all crates

---

## Iteration 2: CLI Arguments

- [ ] Define `Args` struct with clap derive
- [ ] Add all 5 arguments (input, output, plugin, params, plugin-path)
- [ ] Print parsed arguments to verify

**Test**: `cargo run -- --help` shows usage

---

## Iteration 3: Image I/O

- [ ] Load PNG with `image` crate
- [ ] Convert to `RgbaImage`, extract dimensions
- [ ] Get raw bytes as `Vec<u8>`
- [ ] Save bytes back to PNG output
- [ ] Add test image to `test_images/`

**Test**: `cargo run -- -i test.png -o out.png ...` copies image unchanged

---

## Iteration 4: Plugin Loader

- [ ] Create `plugin_loader.rs` module
- [ ] Implement platform-specific library name (`.dylib`/`.so`/`.dll`)
- [ ] Load library with `libloading`
- [ ] Get `process_image` symbol
- [ ] Call function with SAFETY comment
- [ ] Pass params as `CString`

**Test**: Load plugin, call with test image (plugin does nothing yet)

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
