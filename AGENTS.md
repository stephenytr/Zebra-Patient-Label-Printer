# AGENTS.md - Development Guidelines for label_printer

This document provides guidelines for AI coding agents working in this Rust project.

## Project Overview

**Language**: Rust Edition 2021  
**Build System**: Cargo 1.93.0  
**Primary Binary**: `src/main.rs` (192 lines)  
**Purpose**: Zebra label printer interface for patient identification labels

## Build, Test, and Lint Commands

### Building

```bash
# Development build (unoptimized, with debug symbols)
cargo build

# Release build (optimized)
cargo build --release

# Check for compilation errors without building
cargo check
```

### Running

```bash
# Build and run in debug mode
cargo run

# Build and run optimized release build
cargo run --release

# Pass arguments to the application
cargo run -- [ARGS]
```

### Testing

```bash
# Run all tests (currently none defined)
cargo test

# Run specific test by name
cargo test [test_name]

# Run tests with stdout/stderr output visible
cargo test -- --nocapture

# Run tests sequentially (not parallel)
cargo test -- --test-threads=1

# Compile tests without running them
cargo test --no-run
```

**Current State**: No test functions exist yet. To add tests:
- Add `#[cfg(test)]` module in `src/main.rs`, OR
- Create `tests/` directory for integration tests

### Code Quality

```bash
# Format code according to Rust standards
cargo fmt

# Check formatting without modifying files
cargo fmt --check

# Lint code for common mistakes and improvements
cargo clippy

# Lint all targets (tests, examples, benchmarks)
cargo clippy --all-targets
```

### Documentation

```bash
# Generate documentation
cargo doc

# Generate and open documentation in browser
cargo doc --open
```

### Cleanup

```bash
# Remove build artifacts from target/ directory
cargo clean
```

## Code Style Guidelines

### Import Organization

Organize imports by source, external crates first, then standard library:

```rust
// External crates
use chrono::{Local, NaiveDate};

// Standard library (grouped by module)
use std::fs::{self, OpenOptions};
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};
```

**Pattern**: Use `{self, Item}` syntax for multiple imports from the same module.

### Naming Conventions

- **Functions and variables**: `snake_case`
  - Examples: `read_input()`, `format_date_ddmmyyyy()`, `detect_printers()`
- **Types (structs, enums)**: `PascalCase`
  - Example: `PrinterDevice`
- **Constants**: `SCREAMING_SNAKE_CASE`
  - Example: `const ZEBRA_VENDOR_ID: &str = "0a5f";`
- **Make names descriptive**: Function names should indicate purpose clearly

### Type Usage

- Use `PathBuf` for owned paths, `&Path` for borrowed paths
- Use `String` for owned strings, `&str` for string slices
- Leverage Rust's type system - avoid raw types when semantic types exist
- Apply `#[derive(Debug)]` to structs for debugging support

### Error Handling (CRITICAL)

Use appropriate error handling patterns based on context:

**1. Result with `?` operator (preferred for I/O operations):**
```rust
fn print_label(zpl: &str, printer_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().write(true).open(printer_path)?;
    file.write_all(zpl.as_bytes())?;
    file.flush()?;
    Ok(())
}
```

**2. Option type for nullable values:**
```rust
fn format_date_ddmmyyyy(input: &str) -> Option<String> {
    if let Ok(_parsed_date) = NaiveDate::parse_from_str(...) {
        return Some(format!(...));
    }
    None
}
```

**3. Match expressions for error recovery:**
```rust
match print_label(&zpl, &printer_path) {
    Ok(_) => println!("✓ Success"),
    Err(e) => eprintln!("✗ Error: {}", e),
}
```

**4. Unwrap/Expect sparingly (only for truly exceptional cases):**
```rust
// Only use when failure is truly unrecoverable
io::stdout().flush().unwrap();
let num: i32 = input.parse().expect("Please enter a valid integer");
```

### Formatting

- **Indentation**: 4 spaces (Rust standard)
- **Line length**: Keep under 100 characters when reasonable
- **Blank lines**: Use to separate logical sections
- **String literals**: Use raw strings `r#"..."#` for multi-line content
- **String formatting**: Use `format!()` macro for interpolation

### Functional Patterns

Leverage Rust's functional programming features:

```rust
// Iterator chains with combinators
detect_printers()
    .into_iter()
    .find(|p| p.vendor_id == ZEBRA_VENDOR_ID)
    .map(|p| p.device_path)

// Option/Result combinators
fs::read_to_string(path).ok().map(|s| s.trim().to_string())
```

### Comments

- **Minimal comments**: Let code be self-documenting through clear function names
- **Use comments only when**: Logic is non-obvious or complex
- **No doc comments (`///`) currently**: Add them when creating library APIs
- **Avoid stating the obvious**: Don't comment what the code clearly shows

## Dependency Management

### Current Dependencies

```toml
[dependencies]
chrono = "0.4"  # Date and time handling
```

### Adding Dependencies

```bash
# Add a new dependency
cargo add [package_name]

# Add with specific version
cargo add [package_name]@1.0

# Add as dev dependency (tests only)
cargo add --dev [package_name]
```

**Philosophy**: Minimize external dependencies. Prefer standard library when possible.

## Project-Specific Context

### Hardware Integration

This application interfaces with Zebra label printers via USB:
- Detects printers through `/sys/class/usbmisc` 
- Vendor ID: `0a5f` (Zebra)
- Default device path: `/dev/usb/lp0`

### ZPL Format

Labels use Zebra Programming Language (ZPL):
- Generated via `generate_zpl()` function
- Uses raw string literals for multi-line templates
- Fixed positioning with `^FO` commands

## Version Control

Current `.gitignore` excludes:
- `/target` (build artifacts)
- `data` (local data directory)
- `~/.undodir` (editor undo files)
- `Cargo.lock` (currently ignored, consider including for binary projects)

## When Making Changes

1. **Run `cargo check`** before committing to catch compilation errors
2. **Run `cargo fmt`** to format code according to Rust standards
3. **Run `cargo clippy`** to catch common mistakes and improve code quality
4. **Test manually** with `cargo run` if adding new features
5. **Consider adding tests** for new functionality (none exist yet)

## No Configuration Files

This project uses default Rust tooling:
- No custom `rustfmt.toml` (uses default formatting)
- No custom `clippy.toml` (uses default lints)
- No `rust-toolchain.toml` (uses system default)

Follow standard Rust conventions as enforced by `rustfmt` and `clippy`.
