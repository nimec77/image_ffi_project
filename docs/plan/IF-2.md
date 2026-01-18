# Plan: IF-2 - CLI Arguments

**Status:** PLAN_APPROVED

## Overview

Implement command-line argument parsing for the image processing CLI application using clap derive macros. This replaces the current stub `main.rs` with a proper CLI interface that accepts all 5 required arguments.

---

## Components

### 1. Args Struct

**Location:** `image_processor/src/main.rs`

A single struct that defines all CLI arguments using clap's derive API:

```rust
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
```

### 2. Main Function Update

**Location:** `image_processor/src/main.rs`

Update main function to:
- Return `anyhow::Result<()>` for future error handling
- Parse arguments using `Args::parse()`
- Print parsed arguments for verification (temporary)

---

## API Contract

### CLI Interface

| Argument | Flag | Type | Required | Default | Description |
|----------|------|------|----------|---------|-------------|
| input | `--input` | PathBuf | Yes | - | Path to the original PNG image |
| output | `--output` | PathBuf | Yes | - | Path to save the processed image |
| plugin | `--plugin` | String | Yes | - | Plugin name without extension |
| params | `--params` | PathBuf | Yes | - | Path to JSON parameters file |
| plugin_path | `--plugin-path` | PathBuf | No | `target/debug` | Directory containing plugins |

### Usage Examples

```bash
# Full invocation
cargo run -- --input test.png --output out.png --plugin mirror_plugin --params params.json --plugin-path ./plugins

# With default plugin path
cargo run -- --input test.png --output out.png --plugin mirror_plugin --params params.json

# Display help
cargo run -- --help
```

---

## Data Flows

```
User Input (CLI)
       |
       v
+------------------+
|   clap Parser    |  <- Parses command line arguments
|   Args::parse()  |
+------------------+
       |
       v
+------------------+
|   Args struct    |  <- Typed, validated arguments
+------------------+
       |
       v
+------------------+
|  Print to stdout |  <- Verification (temporary)
+------------------+
       |
       v
+------------------+
|    Return Ok(()) |  <- Success
+------------------+
```

**Error Flow (handled by clap):**
- Missing required argument -> clap prints error message and exits
- Invalid argument -> clap prints error message and exits
- `--help` flag -> clap prints usage and exits

---

## Non-Functional Requirements (NFRs)

| NFR | Requirement | How Addressed |
|-----|-------------|---------------|
| Usability | Clear help output | clap auto-generates help from doc comments |
| Maintainability | Simple, readable code | Use derive macros, no custom parsing |
| Consistency | Follow project conventions | Use `anyhow::Result`, follow KISS principle |
| Extensibility | Easy to add arguments later | Derive-based struct allows simple field addition |

---

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| clap API changes | Low | Low | clap 4.x is stable; pinned in Cargo.toml |
| Argument naming mismatch with future usage | Low | Medium | Names align with docs/idea.md specification |
| Confusion between `--plugin-path` (kebab-case) and `plugin_path` (snake_case) | Low | Low | clap handles conversion automatically |

---

## Implementation Steps

1. **Add imports** to `main.rs`:
   - `use anyhow::Result;`
   - `use clap::Parser;`
   - `use std::path::PathBuf;`

2. **Define Args struct** with:
   - `#[derive(Parser)]` attribute
   - 5 fields with appropriate types
   - Doc comments for each field (become --help text)
   - `#[arg(long)]` for all fields
   - `#[arg(default_value = "target/debug")]` for plugin_path

3. **Update main function**:
   - Change signature to `fn main() -> Result<()>`
   - Call `Args::parse()` to get arguments
   - Print each argument for verification
   - Return `Ok(())`

4. **Verification**:
   - Run `cargo build` to ensure compilation
   - Run `cargo run -- --help` to verify help output
   - Run with all arguments to verify parsing
   - Run without `--plugin-path` to verify default

---

## Files to Modify

| File | Change |
|------|--------|
| `image_processor/src/main.rs` | Replace stub with CLI argument parsing |

No new files need to be created. No other files are affected.

---

## Out of Scope

The following are explicitly NOT part of this iteration:
- File existence validation
- Image loading/processing
- Plugin loading
- Parameter file reading
- Logging initialization (may be added but not required)

---

## Acceptance Criteria

1. `cargo build` completes without errors
2. `cargo run -- --help` displays usage with all 5 arguments
3. All arguments can be parsed and printed correctly
4. `--plugin-path` defaults to `target/debug` when not specified
5. Missing required arguments produce helpful error messages

---

## Open Questions

None - all requirements are clearly specified in the PRD and research documents.

---

## Alternatives Considered

### Alternative 1: Positional Arguments
Instead of `--input file.png`, use positional arguments like `image_processor file.png out.png mirror_plugin params.json`.

**Decision:** Rejected. Named arguments are more explicit and self-documenting, especially with 5 arguments.

### Alternative 2: Short Flags
Add short flags like `-i` for `--input`, `-o` for `--output`.

**Decision:** Deferred. Per KISS principle, start with long flags only. Can add short flags later if users request them.

### Alternative 3: Environment Variables
Allow configuration via environment variables in addition to CLI arguments.

**Decision:** Out of scope. Not in requirements, adds complexity.
