# moire_pattern

A Rust program that generates [Moiré patterns](https://en.wikipedia.org/wiki/Moir%C3%A9_pattern) — interference patterns produced when two similar grids or line sets are overlaid at a slight offset or angle.

## About

This project generates Moiré patterns using Rust. It overlays repeating grids with slight offsets to create cool interference effects, and uses randomness using the rand crate to keep things interesting.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024)
- Cargo (included with Rust)

## Getting Started

Clone the repository:

```bash
git clone https://github.com/Sounak008/moire_pattern.git
cd moire_pattern
```

Build and run:

```bash
cargo run
```

## Dependencies

| Crate | Version | Purpose          |
|-------|---------|------------------|
| rand  | 0.8     | Random number generation |

## Project Structure

```
moire_pattern/
├── src/
│   └── main.rs
├── Cargo.toml
└── Cargo.lock
```

## License

Free to use, do whatever you want with it.