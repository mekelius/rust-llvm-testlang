# Notes

## Inkwell and llvm
- had to ```sudo apt install libpolly-22-dev``` to get inkwell to compile
- also ```sudo apt install libz-dev``` or perhaps ```libzstd-dev```

## Rust and cargo
- Nice feature in cargo: Rust files under src/bin are automatically compiled to separate binaries
    - run with ```cargo run --binary MY_BINARY```
