# Rust Bindings

You can use [rustup](https://rustup.rs/) to install Rust on your system.
Currently, the examples only run on x86_64/Linux.

## Build Requirements

The API bindings are generated on the fly so that they can take into account
conditional compilation based on the platform. Therefore, Clang is required to
create the bindings as detailed [here][clang].

[clang]: https://servo.github.io/rust-bindgen/requirements.html

## Runtime Requirements

In order the run the code the edk library must be installed. Here is an example
of how to install edk on Linux:

```sh
$ sudo ln -s /absolute/path/to/community-sdk/bin/linux64/libedk.so $LD_LIBRARY_PATH
```

## Running an Example

You can run can example using `cargo run --bin <example_name>`, for example
```sh
$ cargo run --bin average_band_powers
```
