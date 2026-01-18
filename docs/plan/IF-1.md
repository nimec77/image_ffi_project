# Plan: IF-1 - Iteration 1: Project Setup

**Status:** DRAFT

## Summary

Set up the Cargo workspace scaffolding for the Image FFI Project with three crates: `image_processor` (main CLI app), `mirror_plugin`, and `blur_plugin`. All crates should compile successfully with minimal stub implementations.

---

## Components

### 1. Workspace Root (`Cargo.toml`)

- Defines workspace with three members
- Uses resolver version 2

### 2. Main Application (`image_processor`)

| File | Purpose |
|------|---------|
| `Cargo.toml` | Dependencies: clap (derive), image, libloading, log, env_logger, anyhow |
| `src/main.rs` | Minimal entry point (hello world) |
| `src/plugin_loader.rs` | Empty stub file |

### 3. Mirror Plugin (`mirror_plugin`)

| File | Purpose |
|------|---------|
| `Cargo.toml` | cdylib crate with serde (derive), serde_json, log |
| `src/lib.rs` | Stub `process_image` FFI function |

### 4. Blur Plugin (`blur_plugin`)

| File | Purpose |
|------|---------|
| `Cargo.toml` | cdylib crate with serde (derive), serde_json, log |
| `src/lib.rs` | Stub `process_image` FFI function |

---

## API Contract

### FFI Function Signature (Plugins)

Both plugins export:

```rust
#[no_mangle]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
)
```

For this iteration, the function body will be empty (no-op).

---

## Data Flows

```
N/A for this iteration - only scaffolding, no actual data processing.
```

Build flow:
```
cargo build
    ├── image_processor → target/debug/image_processor (binary)
    ├── mirror_plugin  → target/debug/libmirror_plugin.dylib
    └── blur_plugin    → target/debug/libblur_plugin.dylib
```

---

## Non-Functional Requirements

| Requirement | Target |
|-------------|--------|
| Build success | `cargo build` exits with code 0 |
| Warnings | 0 (or minimal, allow dead_code for stubs) |
| Plugin output | Dynamic libraries generated in target/debug/ |

---

## Implementation Steps

### Step 1: Create Workspace Root `Cargo.toml`

```toml
[workspace]
members = [
    "image_processor",
    "mirror_plugin",
    "blur_plugin",
]
resolver = "2"
```

### Step 2: Create `image_processor` Crate

1. Create `image_processor/Cargo.toml`:
   - Package name: `image_processor`
   - Edition: 2024
   - Dependencies: clap (features=["derive"]), image, libloading, log, env_logger, anyhow

2. Create `image_processor/src/main.rs`:
   - Minimal main function that prints "Hello, world!" or similar
   - No CLI parsing yet

3. Create `image_processor/src/plugin_loader.rs`:
   - Empty file (stub for future implementation)

### Step 3: Create `mirror_plugin` Crate

1. Create `mirror_plugin/Cargo.toml`:
   - Package name: `mirror_plugin`
   - Edition: 2024
   - `crate-type = ["cdylib"]`
   - Dependencies: serde (features=["derive"]), serde_json, log

2. Create `mirror_plugin/src/lib.rs`:
   - Import `std::ffi::c_char`
   - Stub `process_image` function with correct signature
   - Empty body (no-op)

### Step 4: Create `blur_plugin` Crate

1. Create `blur_plugin/Cargo.toml`:
   - Same structure as mirror_plugin

2. Create `blur_plugin/src/lib.rs`:
   - Same structure as mirror_plugin

### Step 5: Verify Build

1. Run `cargo build`
2. Verify exit code 0
3. Check for `.dylib` files in `target/debug/`

---

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Dependency version conflicts | Low | Medium | Use latest stable versions |
| cdylib not generating | Low | High | Verify `crate-type = ["cdylib"]` config |
| Missing feature flags | Medium | Low | Ensure derive features enabled for clap/serde |

---

## Open Questions

None - requirements are fully defined. This is straightforward scaffolding.

---

## References

- `docs/prd/IF-1.prd.md` - PRD for this ticket
- `docs/research/IF-1.md` - Research document
- `docs/vision.md` - Technical architecture
- `docs/conventions.md` - Code conventions
