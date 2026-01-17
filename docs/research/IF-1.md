# Research: IF-1 - Iteration 1: Project Setup

**Status:** RESEARCH_COMPLETE

## Summary

This research document analyzes the requirements and codebase context for setting up the initial Cargo workspace for the Image FFI Project.

---

## Existing Project State

### Current Structure

The project currently contains **documentation only** - no source code or Cargo.toml files exist yet:

```
image_ffi_project/
├── CLAUDE.md                           # Claude Code instructions
├── docs/
│   ├── idea.md                         # Full project requirements
│   ├── vision.md                       # Technical architecture (simplified)
│   ├── conventions.md                  # Code conventions
│   ├── tasklist.md                     # Phase tracking
│   ├── .active_ticket                  # Current ticket: IF-1
│   ├── phase/
│   │   └── phase-1.md                  # Iteration 1 tasks
│   └── prd/
│       └── IF-1.prd.md                 # PRD for this ticket
└── .claude/                            # Claude Code configuration
```

### What Needs to Be Created

All source code and Cargo configuration files need to be created from scratch:

| File | Purpose |
|------|---------|
| `Cargo.toml` | Workspace root defining members |
| `image_processor/Cargo.toml` | Main app dependencies |
| `image_processor/src/main.rs` | Minimal entry point |
| `image_processor/src/plugin_loader.rs` | Stub (can be empty) |
| `mirror_plugin/Cargo.toml` | cdylib configuration |
| `mirror_plugin/src/lib.rs` | Stub process_image function |
| `blur_plugin/Cargo.toml` | cdylib configuration |
| `blur_plugin/src/lib.rs` | Stub process_image function |

---

## Technical Requirements

### Workspace Configuration

The root `Cargo.toml` must define a workspace with three members:

```toml
[workspace]
members = [
    "image_processor",
    "mirror_plugin",
    "blur_plugin",
]
resolver = "2"
```

### Main Application Dependencies

Per `docs/vision.md` and `docs/conventions.md`, `image_processor/Cargo.toml` requires:

| Dependency | Purpose | Notes |
|------------|---------|-------|
| `clap` | CLI argument parsing | Use derive feature |
| `image` | PNG loading/saving | |
| `libloading` | Dynamic library loading | |
| `log` | Logging facade | |
| `env_logger` | Log output | |
| `anyhow` | Error handling | Required per conventions |

### Plugin Dependencies

Both plugins (`mirror_plugin` and `blur_plugin`) require:

| Dependency | Purpose |
|------------|---------|
| `serde` | JSON deserialization | Derive feature |
| `serde_json` | JSON parsing | |
| `log` | Logging | |

### Plugin Configuration

Both plugins must be configured as cdylib:

```toml
[lib]
crate-type = ["cdylib"]
```

---

## FFI Contract

All plugins must export the `process_image` function with this signature:

```rust
#[no_mangle]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
)
```

For stub implementations:
- Function must exist and have correct signature
- Body can be empty or minimal (just return)
- No actual processing logic needed for this iteration

---

## Patterns and Conventions

### From `docs/vision.md`:

1. **KISS Principle**: No premature abstraction
2. **Error Handling**: `anyhow::Result` only, no custom error types
3. **No `lib.rs`** in image_processor - only `main.rs` and `plugin_loader.rs`
4. **No shared crate** - FFI signature duplicated in each plugin
5. **All unsafe code** isolated in `plugin_loader.rs`

### From `docs/conventions.md`:

1. **No bare `.unwrap()`** - use `?` or `.expect("reason")`
2. **SAFETY comments** required for every `unsafe` block
3. **No over-engineering** - solve current problem only

### Deviation from `idea.md`:

Note: `docs/idea.md` suggests `lib.rs` and `error.rs` in image_processor, but `docs/vision.md` explicitly says:
> "No `lib.rs` in image_processor - everything in `main.rs` and `plugin_loader.rs`"
> "No `error.rs` - using anyhow, no custom errors"

**Follow `vision.md`** as it represents the simplified, KISS-compliant architecture.

---

## Dependencies and Layers

### Build Order

No special build order required - Cargo workspace handles parallel builds:
1. All crates can build independently
2. No inter-crate dependencies within the workspace

### Platform Considerations

Plugin library naming differs by platform:
- **macOS**: `libmirror_plugin.dylib`, `libblur_plugin.dylib`
- **Linux**: `libmirror_plugin.so`, `libblur_plugin.so`
- **Windows**: `mirror_plugin.dll`, `blur_plugin.dll`

---

## Limitations and Risks

| Risk | Likelihood | Mitigation |
|------|------------|------------|
| Dependency version conflicts | Low | Use latest stable versions |
| cdylib not generating | Low | Verify `crate-type = ["cdylib"]` in plugin Cargo.toml |
| Missing feature flags | Medium | Ensure `clap` has `derive` feature, `serde` has `derive` feature |

---

## Resolved Questions

| Question | Answer |
|----------|--------|
| Any implementation preferences? | No - proceed with documented structure |

---

## New Technical Questions

None discovered - the requirements are well-defined for this scaffolding iteration.

---

## Implementation Checklist

For IF-1 implementation, ensure:

- [ ] Root `Cargo.toml` defines workspace with all 3 members
- [ ] `image_processor/Cargo.toml` includes all 6 dependencies
- [ ] Plugin `Cargo.toml` files have `crate-type = ["cdylib"]`
- [ ] Plugin `Cargo.toml` files include serde, serde_json, log
- [ ] `main.rs` is minimal and compiles
- [ ] `plugin_loader.rs` can be empty/stub
- [ ] Plugin `lib.rs` files have stub `process_image` with correct FFI signature
- [ ] `cargo build` completes with exit code 0
- [ ] Plugin `.dylib` files appear in `target/debug/`

---

## References

- `docs/idea.md` - Full project requirements
- `docs/vision.md` - Technical architecture
- `docs/conventions.md` - Code conventions
- `docs/phase/phase-1.md` - Iteration 1 tasks
- `docs/prd/IF-1.prd.md` - PRD for this ticket
