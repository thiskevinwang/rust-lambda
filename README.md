# Rust Lambda Functions

## Prerequisites

[Rustup](https://www.rust-lang.org/tools/install)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Install _the target_

```sh
rustup target add x86_64-unknown-linux-musl
```

Install a _linker_

```sh
brew install filosottile/musl-cross/musl-cross
```

Tell Cargo to use the linker

```sh
$ mkdir .cargo
$ echo '[target.x86_64-unknown-linux-musl]
linker = "x86_64-linux-musl-gcc"' > .cargo/config
```

- ⚠️ **TODO:** Still unsure if you run this in the directory or the cargo-generated directories.

## What's inside

```
.
├── README.md
├── my_lambda_function
│   ├── Cargo.lock
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── my_lambda_function_v2
    ├── Cargo.lock
    ├── Cargo.toml
    ├── deploy.sh
    └── src
        └── main.rs
```

### `README.md`

Self Explanitory

### `<some_lambda_function>/`

The root directory `.` holds multiple lambda function directories. These are created by:

```sh
# in the root directory
cargo new <some_lambda_function> --bin
```

## Links / Notes

https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/

- for initial project start

https://robertohuertas.com/2018/12/02/aws-lambda-rust/

- for providing a code snippet that actually works across every stage (Lambda console, API Gateway console, and hitting the deployed endpoint)

For `lambda_runtime ^0.2`, `lambda_runtime::Context` no longer has `.new_error()`. Fix:

```rust
// ❌ doesn't work
Some("error") => Err(c.new_error("Empty first name (body)")),

// ✅ works
Some("error") => Err("Empty first name (body)".into()),
```
