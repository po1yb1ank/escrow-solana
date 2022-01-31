### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```
## Program structure
- src
    - lib.rs -> registering modules
    - entrypoint.rs -> entry of the program
    - instruction.rs  -> program API, (de)serializing instruction data
    - processor.rs -> program logic
    - state.rs -> program objects, (de)serializing state
    - error.rs -> program specific errors
- .gitignore
- Cargo.lock
- Cargo.toml
- Xargo.toml
## Program execution order
- call to the entrypoint
- entrypoint.rs forwards instructions to the processor.rs
- processor.rs asks instructions.rs to decode the instruction_data arg from the entrypoint fun
- aware of decoded instructions processor will decide what processing function to call
- processor may use state.rs to encode state into or decode the state of an account which has been passed into the entrypoint
