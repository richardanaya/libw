# libw

This library is meant to be a more human wrapper around `wasi` and only uses `#[no_std]`. It can also be a great way to learn exactly how `wasi` works!.

# hello world

```rust
fn main() {
    libw::print("hey!\n");
}
```

```make
build:
        @RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-wasi --release
        @cp target/wasm32-wasi/release/my_App.wasm .
```

```bash
wasmer my_app.wasm
```
