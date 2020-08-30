## Setup and start

`cargo run`

If you have problems on Windows with `cargo run` hanging, you may use this from https://github.com/rofrol/rustup-helpers#windows-setup.

## watch

```bash
cargo install cargo-watch
cargo watch -x run
```

## Ubuntu

You may need to install build tools:

```bash
$ sudo apt update
$ sudo apt install build-essential
```

## Debugging

`RUST_BACKTRACE=1 cargo run`

## Compilation time

Compilation time is faster in WSL2 than in git bash in Windows.
