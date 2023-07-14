# POAP Contract

## How to build

Before build the contract make sure you have:
1. Docker installed on your system;
2. The cargo `cargo-run-script` binary. 
This binary can be installed with the `cargo install cargo-run-script` command.

To build the contract from withing the contract directory run:
```shell
cargo optimize
```

This will build the contract and store the compiled wasm code in the `artifacts` directory 
located in the workspace root.
