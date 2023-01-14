# decanter

[![Clippy](https://github.com/FL03/decanter/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/decanter/actions/workflows/clippy.yml)
[![Rust](https://github.com/FL03/decanter/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/decanter/actions/workflows/rust.yml)

***

Welcome to decanter

## Getting Started

Use Rust's built-in package manager [crates](https://crates.io/crates/scsys) to install *scsys*.

### Building from the source

#### *Clone the repository*

```bash
git clone https://github.com/scattered-systems/scsys
cd scsys
```

#### *Build the workspace locally*

```bash
cargo xtask build 
```

or 

```bash
cargo xtask build --release
```

#### *Auto*

Automatically format and analyze the codebase before building then testing.

```bash
cargo xtask auto
```

## Usage

```rust
use scsys::prelude::*;

fn main() {
  println!("{:?}", Message::<String>::default());
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)