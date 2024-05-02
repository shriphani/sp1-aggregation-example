# SP1 Aggregation Example

## Installation

- [Install SP1](https://succinctlabs.github.io/sp1/getting-started/install.html)
- [Install Go 1.22](https://go.dev/doc/install)

## Proof Generation

```
cd script
RUSTFLAGS='-C target-cpu=native' RUST_LOG=info cargo run --release
```

## Export Solidity Verifier
```
cd script
RUSTFLAGS='-C target-cpu=native' cargo run --release --bin verifier
```

## Export Verfification Key
```
cd script
RUSTFLAGS='-C target-cpu=native' RUST_LOG=info cargo run --release --bin vkey
```
