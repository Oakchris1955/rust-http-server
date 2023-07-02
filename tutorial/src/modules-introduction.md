# Library modules

{{#title Library modules}}

All the items of the library we have shown so far are located in the root of the crate. This means that if we wanna import the `Server` struct, we would do something like this:

```rust
# extern crate oak_http_server;
use oak_http_server::Server;
#
# fn main() {
#     /* Your code goes here... */
# }
```

Modules are different: they play a crucial role in organizing and structuring the codebase, while they also serve as a way to group related functions. They help developers keep their code organized, make it easier to reuse code, and allow them to focus on specific parts of their project. For users, modules in a Rust crate make it easy to find and use the features they need while keeping everything organized and straightforward.

In general, items of this crate necessary for the crate to function, as well as widely used items, will be located in the root of the crate, whereas more specific items items will be located within modules.
