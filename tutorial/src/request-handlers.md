# Request handlers

So far we have learned what a HTTP server is capable of, but how exactly do we program one using this library? In this chapter, you will learn how to write custom request handlers and how to use some that the library already provides.

## The handler concept

The usage concept of this library is centered around simplicity: this library shouldn't be pretty complicated in terms of design, but it should also provide broad control to the end user over the HTTP connection.

In order to achieve this, the way the library is built gives full control to the user over a HTTP request and the corresponding response (in the case the request isn't malformed by the user, of course. In that case, the connection is dropped before the user gets the chance to handle the connection)

## What is a handler anyways?

As defined in `lib.rs`, the type `HandlerCallback` is a dynamically dispacted `Fn(Request, Response)`, where `Request` and `Response` and the request and response structs correspondingly. What this basically means is that any function whose has 2 arguments, the first one of which is a `Request` and the second one a `Response` is a valid `HandlerCallback`

For example, the following function falls under the aforementioned criteria:

```rust, no_run
{{#rustdoc_include code-examples/handlers.rs:4:7}}
```

Since our handler is a `Fn` trait and not a concrete `fn` type, we call also pass closures as handlers. The following closure is also a valid handler

```rust, no_run
{{#rustdoc_include code-examples/handlers.rs:10:13}}
```

## Appending handlers

So far we've seen how to create a handler, but we also must let the `Server` struct know of them. Before checking how this is done, let's check at the different types of request handlers, as defined in `lib.rs`:

1) `Any`: This handler will be executed on any requested at the specified target
2) `Specific`: This handler will be executed only on requests made using a specific HTTP method (`GET`, `POST`, etc) at the specified target
3) `Directory`: This handler type requires a seperate chapter on its own and won't be covered here. All you need to know is that it is called not in just one target, but multiple that fall under the same path (a `Directory` handler attached at `/foo` will be called on `/foo/etc`, `/foo/some` and so on)

The `Server` struct provides us with some methods in order to attach handlers. These are the following:

1) `Server::on()` to attach generic `Any` handlers
2) `Server::on_METHOD()` to attach handler for a specific `METHOD` (for example, `on_get` attaches a `Specific` `GET` handler)
3) `Server::on_directory()` to attach a `Directory` handler

All the above function have the same function signature: that means that all of them take a `&str` or `String` as the target argument and a `HandlerCallback` as the second argument

Let's attach those handlers to our server now:

```rust, no_run
{{#rustdoc_include code-examples/handlers.rs:2:}}
```

Similarly, you can create your own handlers. With this library, the sky's the limit to what you can do.
