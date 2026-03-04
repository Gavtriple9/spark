# Spark

**A real-time physics engine written from scratch in Rust.**

Spark is a focused physics simulation library — not a game engine. It aims to deliver accurate, deterministic physics with the performance and safety guarantees of Rust. While it can be embedded into a game engine, its primary goal is precise real-time simulation.

> [!NOTE]
> Spark is in early development. APIs are unstable and features are actively being built.

## Goals

- **Accuracy over approximation** — prioritize physically correct simulation
- **Real-time performance** — suitable for interactive applications at high tick rates
- **Modular architecture** — use only the pieces you need
- **Pure Rust** — no C/C++ dependencies, fully safe where possible

## Project Structure

Spark is organized as a Cargo workspace:

| Crate | Description |
|---|---|
| `spark` | Top-level re-export crate |
| `spark-core` | Core simulation engine (world, bodies, stepping) |
| `spark-common` | Shared types and utilities |

## Quick Start

Add Spark to your project:

```toml
[dependencies]
spark = "0.1.0"
```

Create a simulation world:

```rust
use spark_core::prelude::*;

fn main() {
    let world = World::new("My Simulation");
    // configure bodies, apply forces, step the simulation...
}
```

## Running the Demo

```sh
cargo run --example demo
```

## Building

```sh
cargo build
```

## License

This project is licensed under the [GNU General Public License v3.0](LICENSE).

## Contributing

Contributions are welcome! This project is still early-stage — feel free to open issues or pull requests.
