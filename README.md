# Rust Toolkit

A collection of utility functions and helpers for Rust projects.

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0) [![codecov](https://codecov.io/gh/Revanthshalon/toolkit/graph/badge.svg?token=t0zuCeUcsB)](https://codecov.io/gh/Revanthshalon/toolkit) [![Security Audit](https://github.com/Revanthshalon/toolkit/actions/workflows/audit.yml/badge.svg)](https://github.com/Revanthshalon/toolkit/actions/workflows/audit.yml) [![Lint](https://github.com/Revanthshalon/toolkit/actions/workflows/lint.yml/badge.svg)](https://github.com/Revanthshalon/toolkit/actions/workflows/lint.yml) [![Format](https://github.com/Revanthshalon/toolkit/actions/workflows/format.yml/badge.svg)](https://github.com/Revanthshalon/toolkit/actions/workflows/format.yml)

## Features

### String Utilities (`stringsx`)
- Case manipulation (uppercase/lowercase initials)
- String coalescing
- String splitting
- UTF-8 safe string truncation

### UUID Utilities (`uuidx`)
- UUID v4 generation

### Error Utilities (`errorsx`)
- Error handling utilities (WIP)

## Quick Start

```toml
# Cargo.toml
[dependencies]
toolkit = { git = "https://github.com/revanthshalon/toolkit" }
```

```rust
use toolkit::stringsx::case::to_upper_initials;
use toolkit::uuidx::new_v4;

fn main() {
    let text = to_upper_initials("hello"); // "Hello"
    let uuid = new_v4();
}
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
