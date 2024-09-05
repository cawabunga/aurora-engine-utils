# Aurora Engine Utils (Rust)

This Rust project provides utilities for working with Aurora Engine transactions. It compiles to WebAssembly (Wasm) to
enable usage in JavaScript environments.

## Building

To build the WebAssembly package:

```
make
```

This will generate a `pkg` directory containing the compiled WebAssembly module and JavaScript bindings.

## JavaScript Usage

For information on how to use the compiled WebAssembly module in JavaScript, please refer to
the [JavaScript README](pkg/README.md).