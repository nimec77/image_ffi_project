# IF-2: CLI Arguments

Status: PRD_READY

## Context / Idea

This iteration focuses on implementing command-line argument parsing for the image processing CLI application. The application needs to accept five arguments that control input/output paths, plugin selection, and configuration.

**From Phase 2 Description:**
- Define an `Args` struct using clap derive macros
- Add all 5 required arguments: input, output, plugin, params, plugin-path
- Print parsed arguments to verify correct parsing
- Acceptance criteria: `cargo run -- --help` shows usage

**From Project Requirements (docs/idea.md):**
The application must accept five command-line arguments:

| Argument | Description |
|----------|-------------|
| `input` | Path to the original PNG image |
| `output` | Path to save the processed image |
| `plugin` | Name of the plugin (dynamic library) without the extension (e.g., `invert`) |
| `params` | Path to a text file with processing parameters |
| `plugin-path` | Path to the directory where the plugin is located (`target/debug` by default) |

**From Technical Vision (docs/vision.md):**
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

## Goals

1. Enable users to specify all required inputs via CLI arguments
2. Provide clear help documentation via `--help` flag
3. Set sensible default for plugin-path (`target/debug`)
4. Establish the CLI interface that will be used throughout the project

## User Stories

1. **As a user**, I want to run the application with `--help` to see all available arguments and their descriptions, so I can understand how to use the tool.

2. **As a user**, I want to specify input and output image paths, so the application knows which image to process and where to save the result.

3. **As a user**, I want to specify a plugin name without file extension, so I can easily switch between different processing plugins.

4. **As a user**, I want to provide a path to a parameters file, so plugins can be configured with custom settings.

5. **As a user**, I want the plugin directory to default to `target/debug`, so I do not need to specify it during development.

## Main Scenarios

### Scenario 1: Display Help
**Given** the application is built
**When** the user runs `cargo run -- --help`
**Then** the application displays usage information with all 5 arguments and their descriptions

### Scenario 2: All Arguments Provided
**Given** the application is built
**When** the user runs:
```bash
cargo run -- --input test.png --output out.png --plugin mirror_plugin --params params.json --plugin-path ./plugins
```
**Then** the application parses all arguments correctly and prints them for verification

### Scenario 3: Default Plugin Path
**Given** the application is built
**When** the user runs without specifying `--plugin-path`:
```bash
cargo run -- --input test.png --output out.png --plugin mirror_plugin --params params.json
```
**Then** the application uses `target/debug` as the default plugin directory

### Scenario 4: Missing Required Arguments
**Given** the application is built
**When** the user runs without required arguments (e.g., no `--input`)
**Then** clap displays an error message indicating which argument is missing

## Success / Metrics

1. **Functional Completeness**: All 5 arguments are defined and parseable
2. **Help Output**: `cargo run -- --help` produces clear, informative output
3. **Default Value**: `plugin-path` defaults to `target/debug` when not specified
4. **Build Success**: `cargo build` completes without errors
5. **Verification Output**: Parsed arguments are printed to stdout for manual verification

## Constraints and Assumptions

### Constraints
- Must use `clap` crate with derive macros (already added to Cargo.toml)
- Must follow KISS principle - no premature abstraction
- Arguments should use `PathBuf` for file paths, `String` for plugin name
- All code changes in `image_processor/src/main.rs`

### Assumptions
- Phase 1 (project setup) is complete - workspace and dependencies are configured
- clap 4.x with derive feature is available (confirmed in Cargo.toml)
- No file existence validation required at this phase (just parsing)
- Argument printing is temporary for verification, will be removed in later phases

### Technical Decisions
- Use positional-style arguments with long flags (e.g., `--input`, `--output`)
- Use clap's derive API for type-safe argument parsing
- Return `anyhow::Result` from main for future error handling

## Risks

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| clap API changes between versions | Low | Low | clap 4.x is stable; derive API is well-documented |
| Argument naming conflicts with future features | Low | Medium | Following established naming from docs/idea.md |

## Open Questions

None - all requirements are clearly specified in docs/idea.md and docs/vision.md.
