# Installation

## Prerequisites

Make sure that you have the following installed:

- [The Rust programming language](https://www.rust-lang.org/tools/install)
- [Cargo](https://github.com/rust-lang/cargo), which should be automatically installed with Rust

To check if you already have all of the above installed, run the following commands:

1) `rustup --version` and `rustc --version` to check whether Rust is installed
2) `cargo --version` for Cargo

## Project setup

Begin by creating a new Cargo package. Do this by running `cargo new oak-http-tutorial`, which will create a new directory named `oak-http-tutorial`, containing a Cargo package and a [Git](https://git-scm.com/) repository.

`cd` to the new directory by using `cd oak-http-tutorial`.

## Add OakHTTP as a project dependency

There are two (2) ways to add a package published in <https://crates.io/> as a project dependency.

1) Run `cargo add oak-http-server` while on the project directory. After a few seconds, the latest version of the `OakHTTP` library should now be added as a dependency of your project
2) Open the `Cargo.toml` in the project's root and add a new line under the `[dependencies]` section (create one if it doesn't exist) containing the string `oak-http-server = "VERSION"`, where `VERSION` is which version of the library to install (check all the available versions at <https://crates.io/crates/oak-http-server/versions>)
