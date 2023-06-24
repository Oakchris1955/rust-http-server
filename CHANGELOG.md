# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2023-06-23

Handler and docs update

### Added

- New handler type: `Directory`
- New module: `handlers`. This module contains various `Directory` type handlers
- New method to append `Directory` type handlers
- New type alias, `Headers`, which is a `HashMap<String, String>`
- New example named `file_server` that utilises the latest changes to create a function file server

### Changed

- Document every part of the crate that wasn't documented before
- Change `HandlerCallback` type to a dynamically dispatched `Fn` trait. Also, this type is now contained inside a container
- Change `Server` struct's `on-*` method definitions accordingly (Note: this change shoudln't have an impact on the way users use those methods)
- Replace all `Vec<Header>` occurrences with the new type alias, `Headers` and update code accordingly

### Removed

- Removed `Header` struct

## [0.2.0] - 2023-06-09

Renaming update

### Changed

- Rename the majority of crate items. Items like `HttpServer` are now simply called `Server`
- Update examples accordingly

### Removed

- Remove `QueryParameter` struct and replace it with a `HashMap<String, String>` wherever used

## [0.1.0] - 2023-06-06

Initial Release

Significants changes made before this release:

- Project was initially a pretty basic binary HTTP server, then turned into a library, which is easier to maintain
- Some basic astructs and enums were created
- An `/examples` directory was made, to help illustrate how to use the library

[0.3.0]: https://github.com/Oakchris1955/rust-http-server/compare/v0.2.0..v0.3.0
[0.2.0]: https://github.com/Oakchris1955/rust-http-server/compare/v0.1.0..v0.2.0
[0.1.0]: https://github.com/Oakchris1955/rust-http-server/releases/tag/v0.1.0
