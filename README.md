# SP1 Aggregation Example

## Installation

- [Install SP1](https://succinctlabs.github.io/sp1/getting-started/install.html)
- [Install Go 1.22](https://go.dev/doc/install)

## Proof Generation

```
cd script
RUSTFLAGS='-C target-cpu=native' RUST_LOG=info cargo run --release
```

If you are on an AVX-512 machine, you can use the flags `RUSTFLAGS='-C target-cpu=native -C target_feature=+avx512ifma,+avx512vl'` instead.

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
