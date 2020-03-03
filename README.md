# photo-album

[![master](https://circleci.com/gh/rcoy-v/photo-album/tree/master.svg?style=svg)](https://circleci.com/gh/rcoy-v/photo-album/tree/master)

A application for fetching photos by album id. Written in Rust.

This uses https://jsonplaceholder.typicode.com as the photo store.

## Running Locally

There are multiple options for running this project.
This will mostly depend on what tools you want to install.

Most commands will refer to `make`.
If `make` is not installed, copy the commands in [makefile](makefile),
and run them directly in the terminal.

### docker

This only requires that [Docker](https://docs.docker.com/) be installed and running.

1. `make docker`
1. `docker run --rm rcoy-v/photo-album 1`

Building the Docker image runs linter, format check, and all tests
before building a binary with release profile.
This will take a few minutes the first time it is ran.

### source

To run everything from source, the following tools need to be setup:

1. Install [rustup](https://www.rust-lang.org/tools/install). 
`rustup` manages installations of`rustc` and `cargo`.
1. Add `~/.cargo/bin` to `$PATH`, if not done so during `rustup` install.
1. Verify everything is installed correctly.
    1. `rustc --version`
    1. `cargo --version`
1. `rustup component install clippy-preview`. Installs the linter.
1. `rustup component install rustfmt`. Install the formatter.

Now all tools are installed to run everything:

#### make
```bash
make
./target/debug/photo-album 1
```
Builds an unoptimized binary for testing.
Much faster than building an actual release.

#### make release

```bash
make release
./target/release/photo-album 1
```
Builds a fully optimized binary with release profile.
Can take a few minutes.

#### cargo run

```bash
cargo run -- 1
```
Direct way to run application without building and referencing binary.
Everything after `--` is passed directly as arguments to the application.

#### make test

```bash
make test
```
Runs the following:

- Clippy linter
- rustfmt format check
- unit tests
- integration tests

#### make fmt

```bash
make fmt
```
Fixes format issues in code.
