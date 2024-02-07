# no_std tests

This crate makes sure some of the subxt-* crates work in a no-std environment.

We would like it to run in a no-std environment. You can try any of the following to get it to compile:
```
cargo run 
cargo build --target thumbv7em-none-eabi 
cargo build --target aarch64-unknown-none
```
Currently it does not compile due to linker errors and I have no idea how to resovle these. 