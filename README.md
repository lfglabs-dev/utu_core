# Utu Core

Built with Rust, Utu Core connects to Bitcoin and Ordinals clients allowing to create secure proofs of Bitcoin transactions and events for the Utu relaying contract. Named after the ancient Sumerian sun god who was believed to see everything that happened in the world, Utu aims to provide transparency and security in the Bitcoin ecosystem by bridging these technologies.

## Repository Structure

This workspace is organized into multiple Rust crates:

1. **utu_core**: Contains the common code and core functionality of Utu.
2. **utu_http_server**: Implements the HTTP server for Utu.

## Installing Dependencies

1. **dotenvx**: Follow the installation instructions at [dotenvx installation guide](https://dotenvx.com/docs/install).
2. **cargo**: Ensure you have the Rust package manager installed.

## Running the Application

To start the HTTP server:
```bash
dotenvx run -f .env -- cargo run --bin utu_http_server
```

## WebAssembly Integration

The `utu_core` can be compiled to WebAssembly, allowing its core features to be used directly in JavaScript applications. To build `utu_core` for usage in JavaScript, use the following command:

```bash
wasm-pack build utu_core --target bundler --no-default-features
```

## Testing

To test `utu_core`, run:

```bash
cargo test -p utu_core
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
