# Request handlers

So far we have learned what a HTTP server is capable of, but how exactly can we program one? In this chapter, you will learn how to write your own request handlers and how to use some that the library already provides.

## The handler concept

The whole usage concept of this library is centered around simplicity: this library shouldn't be pretty complicated in terms of design, but it should also provide broad control to the end user over the HTTP connection. We have taken care of all the complicated stuff so that you can focus only on what matters for you

In order to achieve this, the library is built in a way that gives full control to the developer over a HTTP request and the corresponding response (if the request isn't malformed by the client, of course. In that case, the connection is dropped before the program gets the chance to even handle the connection. For the program, it would be as if the request was never received)

## What is a handler anyways?

As defined in `lib.rs`, the type `HandlerCallback` is a dynamically dispacted `Fn(Request, Response) -> io::Result<()>`, where `Request` and `Response` and the request and response structs correspondingly. What this basically means is that any function whose has 2 arguments, the first one of which is a `Request`, the second one a `Response` and which returns an `io::Result<()>` is a valid `HandlerCallback`

For example, the following function falls under the aforementioned criteria:

```rust, no_run
{{#rustdoc_include code-examples/handlers.rs:5:8}}
```

Since our handler is a `Fn` trait and not a concrete `fn` type, we call also pass closures as handlers. The following closure is also a valid handler

```rust, no_run
{{#rustdoc_include code-examples/handlers.rs:11:15}}
```

### About error handling

The reason `HandlerCallback` is defined having an `io::Result<()>` return type is in order to easily handler write errors. As you will see later, various `Response` methods also return `io::Result<()>`. This means that you have to either handle these errors yourself when calling those methods (not recommended) or you can just use the `?` operator after the method. When a handler returns an `io::Error` instead of `Ok(())`, the error is handled by the server. This means that all you have is to make sure that the handler returns an `io::Result<()>` (just add `Ok(())` at the end of the handler) and put the `?` operator after those methods so that the library can handle the error were it to occur

## Appending handlers

So far we've seen how to create a handler, but we must also append them to the `Server` struct. Before checking how this is done, let's check at the different types of request handlers, as defined in `lib.rs`:

1) `Any`: This handler will be executed on any request at the specified target
2) `Specific`: This handler will be executed only for requests made using a specific HTTP method (`GET`, `POST`, etc) at the specified target
3) `Directory`: This handler type requires a seperate chapter on its own and won't be covered in here. All you need to know is that it is called not in just one target, but multiple that fall under the same path (a `Directory` handler attached at `/foo` will be called on `/foo/etc`, `/foo/some` and so on)

The `Server` struct provides us with several methods to attach our handlers. Those are:

1) `Server::on()` to attach generic `Any` handlers
2) `Server::on_METHOD()` to attach handler for a specific `METHOD` (for example, `on_get` attaches a `Specific` `GET` handler)
3) `Server::on_directory()` to attach a `Directory` handler

All the above function have the same function signature: that means that all of them take a `&str` or `String` as the target argument and a `HandlerCallback` as the second argument

Let's attach those handlers to our server now:

```rust, no_run
{{#rustdoc_include code-examples/handlers.rs:2:}}
```

Likewise, you can create and append your own handlers. With this library, the sky's the limit to what you can do.
