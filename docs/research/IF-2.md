# Research: IF-2 - CLI Arguments

**Status:** RESEARCH_COMPLETE

## Summary

This research document analyzes the requirements and codebase context for implementing CLI argument parsing using clap derive macros for the image processing application.

---

## Resolved Questions

| Question | Answer |
|----------|--------|
| Open Questions from PRD | None - PRD states "all requirements are clearly specified in docs/idea.md and docs/vision.md" |
| Implementation preferences | User confirmed: "Use defaults" - proceed with documented requirements |

---

## Related Modules/Services

### Current State of `main.rs`

**File:** `image_processor/src/main.rs`

```rust
fn main() {
    println!("Hello, world!");
}
```

This is a minimal stub from Phase 1 (IF-1). It needs to be replaced with:
1. clap-based argument parsing
2. `anyhow::Result` return type for main
3. Printing of parsed arguments for verification

### Current Dependencies

**File:** `image_processor/Cargo.toml`

All required dependencies are already configured:

| Dependency | Version | Status |
|------------|---------|--------|
| `clap` | 4 (with derive feature) | Ready |
| `image` | 0.25 | Ready (not needed for this ticket) |
| `libloading` | 0.9.0 | Ready (not needed for this ticket) |
| `log` | 0.4 | Ready |
| `env_logger` | 0.11 | Ready |
| `anyhow` | 1 | Ready |

### Related Files

| File | Relevance |
|------|-----------|
| `image_processor/src/plugin_loader.rs` | Stub only, not affected by this ticket |
| `mirror_plugin/src/lib.rs` | Not affected |
| `blur_plugin/src/lib.rs` | Not affected |

---

## Current Endpoints and Contracts

### CLI Interface (To Be Implemented)

Based on `docs/idea.md` and `docs/vision.md`, the application must accept:

| Argument | Type | Required | Default | Description |
|----------|------|----------|---------|-------------|
| `--input` | PathBuf | Yes | - | Path to the original PNG image |
| `--output` | PathBuf | Yes | - | Path to save the processed image |
| `--plugin` | String | Yes | - | Plugin name (without extension) |
| `--params` | PathBuf | Yes | - | Path to JSON parameters file |
| `--plugin-path` | PathBuf | No | `target/debug` | Directory containing plugins |

### Target Args Struct

From `docs/vision.md`:

```rust
#[derive(Parser)]
struct Args {
    /// Path to input PNG image
    input: PathBuf,
    /// Path to save output PNG image
    output: PathBuf,
    /// Plugin name (without extension)
    plugin: String,
    /// Path to JSON parameters file
    params: PathBuf,
    /// Directory containing plugins
    #[arg(default_value = "target/debug")]
    plugin_path: PathBuf,
}
```

**Note:** The PRD shows bare field names, but clap derive convention uses long flags by default. Fields like `input` become `--input` automatically.

---

## Patterns Used

### clap Derive Pattern

Based on `docs/vision.md`, the project uses clap's derive API:

```rust
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    // fields with doc comments for --help
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // use args...
    Ok(())
}
```

### Error Handling Pattern

From `docs/conventions.md`:
- Use `anyhow::Result` for main function return type
- No bare `.unwrap()` calls
- Use `?` operator or `.expect("reason")`

For this iteration, clap handles all argument parsing errors internally with helpful messages, so no explicit error handling code is needed in `main.rs`.

### Logging Pattern (Future)

From `docs/vision.md`, logging should be initialized in main:

```rust
fn main() -> Result<()> {
    env_logger::init();
    // ...
}
```

**Note:** This ticket focuses on argument parsing. Logging initialization may be added here but is not the focus.

---

## Limitations and Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| clap version incompatibility | Low | Low | clap 4.x with derive is stable and well-documented |
| Argument naming mismatch | Low | Medium | Follow exact field names from docs/vision.md |
| PathBuf vs String confusion | Low | Low | Use PathBuf for all file paths, String only for plugin name |
| Missing derive feature | Low | High | Already confirmed in Cargo.toml: `clap = { version = "4", features = ["derive"] }` |

### Scope Boundaries

This ticket does NOT include:
- File existence validation (future iteration)
- Image loading (future iteration)
- Plugin loading (future iteration)
- Parameter file reading (future iteration)

Only argument parsing and verification printing is in scope.

---

## New Technical Questions

### For Follow-up (Not Blocking)

1. **Verification output format:** The PRD says "print parsed arguments to verify correct parsing" but does not specify format. Recommended approach: use Debug trait (`{:?}`) or structured println with field labels.

2. **Long flags vs short flags:** The vision document shows long flags only (`--input`). Consider whether to add short flags (`-i`, `-o`, etc.) for convenience. **Recommendation:** Stick with long flags only per KISS principle.

3. **Help text customization:** clap generates help text from doc comments. Current vision example has minimal doc comments. **Recommendation:** Keep minimal for now, enhance if needed later.

---

## Implementation Checklist

For IF-2 implementation, ensure:

- [ ] Add `use clap::Parser;` and `use std::path::PathBuf;` imports
- [ ] Define `Args` struct with `#[derive(Parser)]`
- [ ] Add all 5 fields with correct types (4 PathBuf, 1 String)
- [ ] Add doc comments for each field (become --help text)
- [ ] Add `#[arg(default_value = "target/debug")]` to plugin_path
- [ ] Change main signature to `fn main() -> anyhow::Result<()>`
- [ ] Parse args with `let args = Args::parse();`
- [ ] Print parsed arguments for verification
- [ ] Return `Ok(())` at end of main
- [ ] Verify `cargo run -- --help` shows all arguments
- [ ] Verify default value for plugin-path works
- [ ] Verify missing required arguments produce error

---

## Code Snippets for Reference

### Minimal Implementation

```rust
use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Path to input PNG image
    #[arg(long)]
    input: PathBuf,

    /// Path to save output PNG image
    #[arg(long)]
    output: PathBuf,

    /// Plugin name (without extension)
    #[arg(long)]
    plugin: String,

    /// Path to JSON parameters file
    #[arg(long)]
    params: PathBuf,

    /// Directory containing plugins
    #[arg(long, default_value = "target/debug")]
    plugin_path: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Input: {:?}", args.input);
    println!("Output: {:?}", args.output);
    println!("Plugin: {}", args.plugin);
    println!("Params: {:?}", args.params);
    println!("Plugin path: {:?}", args.plugin_path);

    Ok(())
}
```

**Note:** The `#[arg(long)]` attribute explicitly enables long flags. Without it, clap may treat fields as positional arguments. Check clap 4.x behavior to confirm.

---

## References

- `docs/idea.md` - Full project requirements (lines 21-34)
- `docs/vision.md` - Args struct example (lines 88-103)
- `docs/conventions.md` - Error handling and KISS principle
- `docs/prd/IF-2.prd.md` - PRD for this ticket
- `image_processor/src/main.rs` - Current stub to be replaced
- `image_processor/Cargo.toml` - Dependencies (already configured)
