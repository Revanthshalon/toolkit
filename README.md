# Rust Toolkit

A collection of utility functions and helpers for Rust projects.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![codecov](https://codecov.io/gh/Revanthshalon/toolkit/graph/badge.svg?token=t0zuCeUcsB)](https://codecov.io/gh/Revanthshalon/toolkit) [![Security Audit](https://github.com/Revanthshalon/toolkit/actions/workflows/audit.yml/badge.svg)](https://github.com/Revanthshalon/toolkit/actions/workflows/audit.yml) [![Lint](https://github.com/Revanthshalon/toolkit/actions/workflows/lint.yml/badge.svg)](https://github.com/Revanthshalon/toolkit/actions/workflows/lint.yml) [![Format](https://github.com/Revanthshalon/toolkit/actions/workflows/format.yml/badge.svg)](https://github.com/Revanthshalon/toolkit/actions/workflows/format.yml)

## Features

### String Utilities (`stringsx`)

The `stringsx` module provides string manipulation utilities:

#### Case Manipulation
- `to_lower_initials`: Converts the first character of a string to lowercase
- `to_upper_initials`: Converts the first character of a string to uppercase

Example:
```rust
use toolkit::stringsx::case::{to_lower_initials, to_upper_initials};

assert_eq!(to_lower_initials("Hello"), "hello");
assert_eq!(to_upper_initials("world"), "World");
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
toolkit = { git = "https://github.com/username/toolkit" }
```

## Usage

```rust
use toolkit::stringsx::case;

fn main() {
    let lowercase = case::to_lower_initials("Hello"); // "hello"
    let uppercase = case::to_upper_initials("world"); // "World"
}
```

## Documentation

The codebase includes comprehensive documentation with examples. You can generate the documentation locally by running:

```bash
cargo doc --open
```

## Testing

Run the test suite with:

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

When contributing to this repository, please first discuss the change you wish to make via issue, email, or any other method with the owners of this repository before making a change.

Please note we have a code of conduct, please follow it in all your interactions with the project.

### Pull Request Process

1. Update the README.md with details of changes to the interface, if applicable.
2. Update any documentation that is affected by your changes.
3. The PR may be merged once you have the sign-off of at least one other developer.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

### MIT License Summary
- ✓ Commercial use
- ✓ Modification
- ✓ Distribution
- ✓ Private use
- ✓ Liability
- ✓ Warranty

The MIT License is a permissive license that is short and to the point. It lets people do anything they want with your code as long as they provide attribution back to you and don't hold you liable.

## Contact

[Revanth Shalon] - [@your_twitter](https://twitter.com/your_twitter) - email@example.com

Project Link: [https://github.com/revanthshalon/toolkit](https://github.com/username/toolkit)
