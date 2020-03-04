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

# working with files

`wasi` modules are only able to work on folders they have been given explicit permission to access. By default `wasi` has 3 basic files:

* 0 - application text input
* 1 - application text output
* 2 - application error text output

The file that you give explicit permission to will be given file descriptor number 3. You must do this manually by specifying the directory.

```bash
wasmer my_app.wasm --dir=. # current directory is given access
```

In `libw` file descriptor 3 is referred to as the `executing directory`.

```rust
let dir = libw::executing_directory();
let mut txt = libw::read_text(dir, "hello.txt");
txt = "goodbye".to_string();
libw::write_text(dir, "hello.txt", txt);
```

# API

* print - print characters
* println - print characters ending with newline
* random - get a random f32
* current_time - get the current time in unix
* exit - stop the current process
* yield_control - yield control of thread
* TODO read_text - read a text file into a directory
* TODO write_text - write a text file into a directory
* environment_variables - get a vector of environment variables
* command_arguments - get a vec of strings of command line arguments
* executing_directory - get the directory the web assembly module was given permission to execute on
