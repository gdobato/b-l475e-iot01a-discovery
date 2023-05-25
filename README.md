# b-l475e-iot01a-discovery

[![ci](https://github.com/gdobato/b-l475e-iot01a-discovery/actions//workflows/ci.yml/badge.svg)](https://github.com/gdobato/b-l475e-iot01a-discovery/actions/workflows/ci.yml) 

Board Support crate for b-l475e-iot01a-discovery

## Installation (Unix-like OS)
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add thumbv7em-none-eabihf
cargo install cargo-embed cargo-binutils
```

## Build
To build an example, run the following command:
```
cargo build --example <example_name> [--release]
```
For instance, to build `blinky`:
```
cargo build --example blinky
```

## Flash with debug probe (JLink, ST-Link)
```
cargo embed --example <example_name> [--release]
```
For example, to flash `blinky`, run the following command:
```
cargo embed --example blinky
```