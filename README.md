# libw

This library is meant to be a more human wrapper around only the strict capabilities of [`wasi`](https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-api.md)

* only uses `#[no_std]` and `alloc` to encourage non-bloated wasm binaries
* high level operations
* great way to learn exactly how `wasi` works!.

# hello world
```toml
[package]
name = "my_app"
version = "0.0.1"

[profile.release]
lto = true

[dependencies]
libw = "0"
```

```rust
fn main() {
    libw::print("hey!\n");
}
```

```make
build:
        @RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-wasi --release
        @cp target/wasm32-wasi/release/my_app.wasm .
```

```bash
wasmer my_app.wasm
```
