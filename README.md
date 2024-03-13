# WASM Component Tutorial

This is a modified version of the wasm component tutorial
[example](https://github.com/bytecodealliance/component-docs/tree/main/component-model/examples/tutorial),
where each component is made into a separate package.

## Building and running the example

To build each component, run the following commands:

```sh
(cd add && cargo component build --release)
(cd sub && cargo component build --release)
(cd calculator && cargo component build --release)
(cd command && cargo component build --release)
```

To compose the components, run the following command:

```sh
wasm-tools compose calculator/target/wasm32-wasi/release/calculator.wasm -d add/target/wasm32-wasi/release/add.wasm -d sub/target/wasm32-wasi/release/sub.wasm -o composed.wasm
wasm-tools compose command/target/wasm32-wasi/release/command.wasm -d composed.wasm -o command.wasm
```

To run the composed component with wasmtime, run the following command:

add:

```sh
wasmtime run command.wasm 5 3 add
```

sub:

```sh
wasmtime run command.wasm 5 3 sub
```
