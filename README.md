![asdsa](https://og-image.now.sh/**Rust**%20Lambda%20Functions.png?theme=light&md=1&fontSize=100px&images=https%3A%2F%2Fassets.vercel.com%2Fimage%2Fupload%2Ffront%2Fassets%2Fdesign%2Fhyper-color-logo.svg&images=https%3A%2F%2Fwww.rust-lang.org%2Fstatic%2Fimages%2Frust-logo-blk.svg&images=https%3A%2F%2Ftechie-jim.net%2Fwp-content%2Fuploads%2F2017%2F07%2Faws-lambda-logo.png&widths=250&widths=350&widths=250&heights=250&heights=350&heights=250)

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

Manually create symlink to the new linker

```sh
ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
```

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

- for providing a code snippet that actually works (with slight modification) across every stage (Lambda console, API Gateway console, and hitting the deployed endpoint)

For `lambda_runtime ^0.2`, `lambda_runtime::Context` no longer has `.new_error()`. Fix:

```rust
// ❌ doesn't work
Some("error") => Err(c.new_error("Empty first name (body)")),

// ✅ works
Some("error") => Err("Empty first name (body)".into()),
```

### &\$%^#! errors...

#### Failing to build with `rusoto` deps, part 1:

> failed to run custom build command for `ring v0.16.14`

Explained by: [This issue comment](https://github.com/briansmith/ring/issues/563#issuecomment-318790822)

Fixed by running:

- documented [here](https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/)

```sh
ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc
```

#### Failing to build with `rusoto` deps, part 2:

> Could not find directory of OpenSSL installation, and this `-sys` crate cannot
> proceed without this knowledge. If OpenSSL is installed and this crate had
> trouble finding it, you can set the `OPENSSL_DIR` environment variable for the
> compilation process.
>
> Make sure you also have the development packages of openssl installed.
> For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.
>
> If you're in a situation where you think the directory _should_ be found
> automatically, please open a bug at https://github.com/sfackler/rust-openssl
> and include information about your system as well as this message.
>
> \$HOST = x86_64-apple-darwin
>
> \$TARGET = x86_64-unknown-linux-musl
>
> openssl-sys = 0.9.57

Fixed in `Cargo.toml`

```diff
[dependencies]
- rusoto_core = "0.43.0"
- rusoto_dynamodb = "0.43.0"

+ [dependencies.rusoto_core]
+ version = "0.43.0"
+ default-features = false
+ features = ["rustls"]

+ [dependencies.rusoto_dynamodb]
+ version = "0.43.0"
+ default-features = false
+ features = ["rustls"]
```
