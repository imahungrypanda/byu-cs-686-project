# BYU CS 686 Project

A Rust project using Verus for formal verification.

## Setup

1. Install Verus by following the instructions at: https://github.com/verus-lang/verus

2. Build and verify your code:

   ```bash
   verus src/main.rs
   ```

   Or if Verus is in a sibling directory:

   ```bash
   ../verus/source/target/debug/verus src/main.rs
   ```

## Project Structure

- `src/main.rs` - Main source file with Verus verification code
- `Cargo.toml` - Rust project configuration with Verus dependencies

## Resources

- [Verus Documentation](https://verus-lang.github.io/)
- [Verus GitHub](https://github.com/verus-lang/verus)
