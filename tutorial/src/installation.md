# Installation

Before using the library, you should install it first.

## Prerequisites

Make sure that you have the followings installed:

- [The Rust programming language](https://www.rust-lang.org/tools/install)
- [Cargo](https://github.com/rust-lang/cargo), which should come along with Rust by default

To check if you already have all the above installed, run the following commands:

1) `rustup --version` and `rustc --version` to check whether Rust is installed
2) `cargo --version` for Cargo

If all the above commands run successfully and return without an error, that means that they are successfully installed

## Project setup

Once you have installed both off them, you must create a new Cargo package. Do this by running `cargo new oak-http-tutorial`, which will create a new directory named `oak-http-tutorial`, containing a Cargo package and a [Git](https://git-scm.com/) repository.

Now that you have created a new package, `cd` to it using `cd oak-http-tutorial`.

## Add OakHTTP as a project dependency

There are two (2) ways to add a package published at <https://crates.io/> as a project dependency.

1) Run `cargo add oak-http-server` while on the same directory as the project. After a few seconds, the latest version of the OakHTTP library should now be a dependency of your project
2) Open the `Cargo.toml` in the project's root and add a new line in the follwoing format under the `[dependencies]` section (create one if it doesn't exist) containing the string `oak-http-server = "VERSION"`, where `VERSION` is which version of the library to install (check all available versions at <https://crates.io/crates/oak-http-server/versions>)
