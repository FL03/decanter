# decanter

[![Clippy](https://github.com/FL03/decanter/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/decanter/actions/workflows/clippy.yml)
[![Rust](https://github.com/FL03/decanter/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/decanter/actions/workflows/rust.yml)
[![crates.io](https://img.shields.io/crates/v/decanter.svg)](https://crates.io/crates/decanter)
[![docs.rs](https://docs.rs/decanter/badge.svg)](https://docs.rs/decanter)

***

Welcome to decanter

## Getting Started

Use Rust's built-in package manager [crates](https://crates.io/crates/decanter) to add *decanter*.

### Building from the source

#### *Clone the repository*

```bash
git clone https://github.com/scattered-systems/scsys
cd scsys
```

#### *Build the workspace locally*

```bash
cargo build --release --workspace
```

or

```bash
cargo build -F wasm --release --target wasm32-unknown-unknown --workspace
```

#### *Testing*

```bash
cargo test --all -F full --release
```

or

```bash
cargo test --all -F wasm --release --target wasm32-unknown-unknown
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