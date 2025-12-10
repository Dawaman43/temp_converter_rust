# Temp Converter (Rust)

A small, focused Rust collection of command-line temperature converter utilities.

This repository contains three small binaries (in `src/bin`) for converting temperatures between Celsius and Fahrenheit and a small wrapper `temp_converter`.

**Highlights:**

- Lightweight, single-purpose CLI tools.
- Simple examples for quick testing.
- Built with stable Rust and Cargo.

**Binaries included:**

- `celsius_to_fahrenheit` — converts Celsius to Fahrenheit.
- `fahrenheit_to_celsius` — converts Fahrenheit to Celsius.
- `temp_converter` — a main convenience binary (see source in `src/main.rs`).

Prerequisites

- Rust (rustup + cargo). Install from https://rustup.rs if needed.

Build & Run (development)

- Build and run with Cargo (development/debug):

```bash
# Run the convenience binary (interactive or argument-based depending on implementation)
cargo run --bin temp_converter

# Run a specific converter with an argument (example):
cargo run --bin celsius_to_fahrenheit -- 37
cargo run --bin fahrenheit_to_celsius -- 98.6
```

Notes:

- If the converters read from STDIN or prompt for input, omit the argument and follow the prompt.
- To run the compiled binary directly after building:

```bash
cargo build
./target/debug/celsius_to_fahrenheit 37
```

Build release

```bash
cargo build --release
./target/release/temp_converter
```

Examples

- Convert 0°C -> °F (approx):

```bash
cargo run --bin celsius_to_fahrenheit -- 0
# expected output: 32
```

- Convert 100°F -> °C (approx):

```bash
cargo run --bin fahrenheit_to_celsius -- 100
# expected output: ~37.78
```

Project layout

- `Cargo.toml` — project manifest
- `src/main.rs` — main binary (convenience wrapper)
- `src/bin/celsius_to_fahrenheit.rs` — standalone converter
- `src/bin/fahrenheit_to_celsius.rs` — standalone converter

Contributing

- Small fixes and clarifying README improvements are welcome. Open a pull request or an issue.
