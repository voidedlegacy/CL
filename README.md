# Cool Language (CL)

This is a work in progress compiler that I started originally in C,
but I decided to rewrite it in rust.

## Usage

Run the executable from a shell with a path to some source code as the only argument. Currently, we print out the furthest progress we are able to make. Eventually, we will output compiled source code.

## Building

### Dependencies: 

- [Rust](https://rustup.rs)

First, we generate the code using Cargo, the official Rust thingy.

```shell
cargo build
```

Finally, run the compiler with the `example` code
```shell
cargo run .\example
```