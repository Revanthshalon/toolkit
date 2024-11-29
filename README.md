# Rust Toolkit

A collection of utility functions and helpers for Rust projects.

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![codecov](https://codecov.io/gh/Revanthshalon/toolkit/graph/badge.svg?token=t0zuCeUcsB)](https://codecov.io/gh/Revanthshalon/toolkit) [![Security Audit](https://github.com/Revanthshalon/toolkit/actions/workflows/audit.yml/badge.svg)](https://github.com/Revanthshalon/toolkit/actions/workflows/audit.yml) [![Lint](https://github.com/Revanthshalon/toolkit/actions/workflows/lint.yml/badge.svg)](https://github.com/Revanthshalon/toolkit/actions/workflows/lint.yml) [![Format](https://github.com/Revanthshalon/toolkit/actions/workflows/format.yml/badge.svg)](https://github.com/Revanthshalon/toolkit/actions/workflows/format.yml)

## Features

### String Utilities (`stringsx`)
- Case manipulation (to_upper_initials/to_lower_initials)
- String coalescing (find first non-empty string)
- String splitting with custom separators
- UTF-8 safe string truncation by byte length

### UUID Utilities (`uuidx`)
- UUID v4 generation
- Random UUID creation

### Error Utilities (`errorsx`)
- Enhanced error handling with context
- Stack trace capture
- Source location tracking
- Status codes and messages
- Error chaining
- Rich error context building

## Quick Start

```toml
# Cargo.toml
[dependencies]
toolkit = { git = "https://github.com/revanthshalon/toolkit" }
```

```rust
use toolkit::stringsx::case::to_upper_initials;
use toolkit::uuidx::new_v4;
use toolkit::errorsx::ErrorX;

fn main() {
    // String manipulation
    let text = to_upper_initials("hello"); // "Hello"

    // UUID generation
    let uuid = new_v4();

    // Error handling
    let err = ErrorX::builder("Operation failed")
        .with_context("User authentication")
        .with_status_code(401)
        .with_status("Unauthorized")
        .build();
}
```

## Examples

### String Utilities
```rust
use toolkit::stringsx::{case, coalesce, split, truncate};

// Case manipulation
assert_eq!(case::to_upper_initials("hello"), "Hello");
assert_eq!(case::to_lower_initials("World"), "world");

// String coalescing
let words = ["", "first", "second"];
assert_eq!(coalesce::coalesce(&words), "first");

// String splitting
assert_eq!(split::splitx("hello world", " "), vec!["hello", "world"]);

// UTF-8 safe truncation
assert_eq!(truncate::truncate_byte_len("Hello,🚧", 7), "Hello,");
```

### Error Handling
```rust
use toolkit::errorsx::ErrorX;
use std::io;

let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
let err = ErrorX::builder("Failed to process file")
    .with_context("Processing user upload")
    .with_source(io_error)
    .with_status_code(500)
    .build();
```

## Documentation

Generate documentation locally:
```bash
cargo doc --open
```

## Contributing

Contributions welcome! Please create an issue or submit a PR.

## License

Apache-2.0 License - See [LICENSE](LICENSE) file

## Contact

Revanth Shalon - [@NautilusTK](https://x.com/NautilusTK) - revanthshalonraj@gmail.com
