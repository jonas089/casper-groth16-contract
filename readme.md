# Example circom verifier

Prerequisite:
[Review this PR](https://github.com/casper-network/casper-node/pull/4894)

This contract constructs an example payload from a real circom proof upon initialization.
The proof is then verified using the experimental host function.

# Compile

```bash
rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
```

out: `target/wasm32-unknown-unknown/release/circom-verifier-casper-example.wasm`

# Optimize

```bash
wasm-opt --strip-debug --signext-lowering circom-verifier-casper-example.wasm circom-verifier-example-optimized.wasm
```

The optimized binary can now be sent to a Casper `2.0` network.