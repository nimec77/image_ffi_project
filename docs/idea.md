# Image FFI Project

A CLI application for image processing with support for dynamically linked plugins via FFI.

## Project Goals

By completing this project, you will gain practical experience with:

- Working with images in Rust
- Dynamically loading and using libraries
- Interacting via FFI and consciously using unsafe code
- Designing an extensible plugin architecture
- Error handling and safe memory management

## Assignment

Write a CLI application that loads an image, applies the specified processing plugin to it, and saves the result.

## Application Architecture

The application must consist of a single binary crate that:

### 1. Command-Line Arguments

Accepts five command-line arguments (the `clap` crate is recommended):

| Argument | Description |
|----------|-------------|
| `input` | Path to the original PNG image |
| `output` | Path to save the processed image |
| `plugin` | Name of the plugin (dynamic library) without the extension (e.g., `invert`) |
| `params` | Path to a text file with processing parameters |
| `plugin-path` | Path to the directory where the plugin is located (`target/debug` by default) |

### 2. Image Loading and Decoding

Using the `image` crate:

- Read a PNG file
- Convert it to RGBA8 format
- Get the width, height, and flattened pixel array in RGBA format (4 bytes per pixel)

### 3. Dynamic Plugin Loading

- By plugin name, find the library file:
  - Linux: `libinvert.so`
  - Windows: `invert.dll`
  - macOS: `libinvert.dylib`
- Search is performed in the directory specified in the `plugin-path` argument
- Load a function with a specific signature from this library (see Plugin API)

### 4. Data Passing to Plugin

Pass the following to the plugin:

- Width and height of the image
- Pointer to an RGBA data array
- Parameter string read from the params file (any text format can be used)

### 5. Saving the Result

- After processing, convert the RGBA array data back to an image
- Save the result to a PNG file at the specified path

## Plugin API

All plugins must export a single function with the following C signature:

```c
void process_image(
    uint32_t width,
    uint32_t height,
    uint8_t* rgba_data,
    const char* params
);
```

### Parameter Description

| Parameter | Description |
|-----------|-------------|
| `width`, `height` | Image dimensions in pixels |
| `rgba_data` | Pointer to a byte array of size `width * height * 4`. Each four bytes represents one pixel in RGBA (Red, Green, Blue, Alpha) format |
| `params` | Parameter string (can be empty or contain JSON, key-value, etc.). The plugin must parse this string itself |

### Important Details

- The plugin modifies data **in-place**, in the same `rgba_data` buffer
- The plugin is responsible for memory management: it must not exceed the array's bounds
- All plugins are written in Rust and compiled to `cdylib`
- The `plugin_loader` module must use the `libloading` crate to dynamically load the library and obtain a pointer to the `process_image` function

## Error Handling

- Check the existence of the input file, the parameter file, and the plugin library
- Handle possible errors when loading an image (invalid format, corrupted data)
- Ensure that pointers passed to the plugin are valid and that the memory is not freed until the plugin completes

## Plugins to Implement

### 1. Mirror Plugin (Image Flip)

Accepts two Boolean parameters:

- `horizontal` — flip horizontally
- `vertical` — flip vertically

### 2. Blur Plugin (Image Blur)

Accepts the parameters:

- `radius` — blur radius
- `iterations` — number of iterations

Algorithm: The new pixel color is equal to the weighted average of the colors of the pixels within the blur radius, where the weight is the distance between the center pixel and the current pixel.

Each plugin should be a separate dynamic library that can be loaded by name.

## Project Structure

```
image_ffi_project/
├── Cargo.toml              # Workspace
├── image_processor/
│   ├── Cargo.toml          # Image processor package
│   └── src/
│       ├── main.rs
│       ├── lib.rs
│       ├── error.rs
│       └── plugin_loader.rs
├── mirror_plugin/          # Mirror plugin package
│   ├── Cargo.toml
│   └── src/lib.rs
├── blur_plugin/            # Blur plugin package
│   ├── Cargo.toml
│   └── src/lib.rs
└── README.md
```

## Pre-Submission Checklist

### Project Build and Structure

- [ ] Repository contains a binary application and at least one plugin
- [ ] Project compiles without errors (`cargo build`)
- [ ] Plugins are built as `cdylib` (`crate-type = ["cdylib"]` in Cargo.toml)
- [ ] Cargo.toml specifies `image`, `clap`, and `libloading` dependencies
- [ ] Code is divided into logical modules (`lib.rs`, `bin/`, `src/plugin_loader.rs`)
- [ ] README.md contains startup description and command examples

### Command Line Arguments

- [ ] Application accepts five arguments: `input`, `output`, `plugin`, `params`, and `plugin-path`
- [ ] `clap` (or equivalent) is used for argument parsing
- [ ] Help is displayed if no arguments are specified
- [ ] Arguments are checked for file existence (input image, parameter file)

### Image Loading and Processing

- [ ] Image is loaded via the `image` crate
- [ ] Image is converted to `Rgba8`
- [ ] Width and height are extracted correctly
- [ ] Pixel array is accessible as a flat RGBA buffer
- [ ] After processing, image is saved as PNG

### Dynamic Plugin Loading

- [ ] `libloading` crate (or equivalent) is used
- [ ] Library is searched by name, taking the OS into account (`.so`, `.dll`, `.dylib`)
- [ ] `process_image` function is loaded with correct signature
- [ ] Load errors are handled (library not found, function not found)

### FFI Safety

- [ ] Call to `process_image` is wrapped in an `unsafe` block
- [ ] Data pointer is obtained safely (`as_mut_ptr()`)
- [ ] Buffer length is calculated correctly (`width * height * 4`)
- [ ] Parameter string is converted to `CString` and passed as raw pointer without leaks
- [ ] Memory is not freed until the plugin exits
- [ ] Application frees memory when plugin exits

### Plugin Examples

- [ ] Mirror plugin is implemented and working
- [ ] Blur plugin is implemented and working
- [ ] Plugins correctly parse parameters from `params`
- [ ] Plugins do not exceed the bounds of the passed buffer
- [ ] Tests for core plugin logic are implemented

### Error Handling

- [ ] All operations with files and libraries return a `Result`
- [ ] Errors are displayed to the user in a user-friendly format
- [ ] Application does not panic on invalid input
- [ ] Resources (file handles, libraries) are freed correctly

## Review Criteria

What reviewers will look for:

1. **Code Cleanliness and Readability**: Meaningful names, no "magic numbers," comments on unsafe blocks
2. **Safety**: Correct pointer conversion, bounds checking, no undefined behavior
3. **Architecture**: Modularization (`plugin_loader`, `error`), code reuse
4. **Error Handling**: All possible errors are handled, no panics (`unwrap()` only in appropriate places)
5. **Following Specifications**: All declared functions are implemented, including loading parameters from a file

## Recommendations

- Parse plugin parameters into a structure (e.g., from JSON)
- Use `log`/`env_logger` instead of `println!` for debug messages
