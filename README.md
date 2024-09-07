# Tests fix for using sp-runtime's `format_runtime_string!` macro

## Build

Build with:

```commandline
 cargo build --release --target wasm32-unknown-unknown 
```

The build will fail at the end because no allocator is defined, but it is enough to show that
the following build error is fixed:

```console
Compile crate with the following crates to get the error:
# error[E0433]: failed to resolve: use of undeclared crate or module `alloc`
```