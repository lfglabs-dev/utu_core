# Utu

Utu is a Rust-based project that connects to Bitcoin or Ordinals clients to create secure proofs of Bitcoin transactions and events. Named after the ancient Sumerian sun god who was believed to see everything that happened in the world, Utu aims to provide transparency and security in the Bitcoin ecosystem.

## Repository Structure

This workspace is organized into multiple Rust crates:

1. **utu_core**: Contains the common code and core functionality of Utu.
2. **utu_http_server**: Implements the HTTP server for Utu.
3. **utu_wasm_module**: Allows core features to be used directly in WebAssembly, enabling integration in web applications.

## Installing Dependencies

1. **dotenvx**: Follow the installation instructions at [dotenvx installation guide](https://dotenvx.com/docs/install).
2. **cargo**: Ensure you have the Rust package manager installed.

## Running the Application

To start the HTTP server:
```bash
dotenvx run -f .env -- cargo run --bin utu_http_server
```

## WebAssembly Integration

The `utu_wasm_module` allows you to use Utu's core features directly in WebAssembly, making it possible to integrate Utu functionality into web applications.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
