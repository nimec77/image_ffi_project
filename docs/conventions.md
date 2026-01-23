# Code Conventions

> Reference: [vision.md](vision.md) for architecture, [idea.md](idea.md) for requirements.

## Core Principle

**KISS** - Keep It Simple, Stupid. No premature abstraction.

## Error Handling

- Use `anyhow::Result` everywhere. No custom error types.
- Never use bare `.unwrap()`. Use `?` or `.expect("clear reason")`.
- Handle all errors gracefully - never panic across FFI boundary.

## Unsafe Code

Every `unsafe` block requires a `// SAFETY:` comment explaining:
- What invariants must hold
- Why those invariants are satisfied

```rust
// SAFETY: rgba_data points to a valid buffer of len bytes, owned by the host
let data = unsafe { std::slice::from_raw_parts_mut(rgba_data, len) };
```

## Dependencies

Only these crates are allowed:
- `clap` - CLI argument parsing
- `image` - PNG loading/saving
- `libloading` - dynamic library loading
- `log` + `env_logger` - logging
- `anyhow` - error handling
- `serde` + `serde_json` - JSON parsing (plugins only)

## Project Structure

```
image_processor/src/
├── main.rs           # Entry point, CLI, image I/O
└── plugin_loader.rs  # All unsafe/FFI code isolated here
```

No `lib.rs`, no `error.rs`, no shared crate.

## FFI Contract

All plugins export this function:
```rust
#[no_mangle]
pub extern "C" fn process_image(
    width: u32,
    height: u32,
    rgba_data: *mut u8,
    params: *const c_char,
) -> i32  // Returns 0 on success, negative error code on failure
```

## Plugin Rules

1. Modify data **in-place** only
2. Never allocate memory the host must free
3. Never read/write beyond `width * height * 4` bytes
4. Parse your own params (JSON format)
5. On error: log and return early, never panic

## Logging

```rust
// main.rs
env_logger::init();

// Anywhere
log::info!("Major step");
log::debug!("Details");
log::error!("Failures");
```

Usage: `RUST_LOG=info ./image_processor ...`

## Code Style

- Meaningful variable names, no magic numbers
- Comments only on unsafe blocks and non-obvious logic
- No over-engineering - solve the current problem only
