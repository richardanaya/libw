# libw

<a href="https://docs.rs/libw"><img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square" alt="docs.rs docs" /></a>

This library is meant to be a more human wrapper around only the strict capabilities of [`wasi`](https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-api.md)

* only uses `#[no_std]` and `alloc` to encourage non-bloated wasm binaries
* does not require rust be built with `wasm32-wasi`
* high level operations independent of POSIX
* great way to learn exactly how `wasi` works!.

# hello world
```toml
[package]
name = "my_app"
version = "0.0.1"

[lib]
crate-type = ["cdylib"]

[profile.release]
lto = true

[dependencies]
libw = "0"
```

```rust
#![no_std]

#[no_mangle]
pub fn _start() {
    libw::print("hey!\n");
}
```

```make
build:
	@RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
	@cp target/wasm32-unknown-unknown/release/wasp.wasm .
```

```bash
wasmer my_app.wasm
```

# API

* print - print characters
* println - print characters ending with newline
* random - get a random f32
* current_time - milliseconds since unix epoc
* unix_time - seconds since unix epoc
* high_preicion_time - get the current realtime value of host clock
* exit - stop the current process
* yield_control - yield control of thread
* TODO read_text - read a text file into a directory
* TODO write_text - write a text file into a directory
* environment_variables - get a vector of environment variables
* command_arguments - get a vec of strings of command line arguments
* executing_directory - get the directory the web assembly module was given permission to execute on
