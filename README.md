# libw

This library is meant to be a more human wrapper around `wasi` and only uses `#[no_std]`. It can also be a great way to learn exactly how `wasi` works!.

# hello world

```rust
fn main() {
    libw::print("hey!\n");
}
```
