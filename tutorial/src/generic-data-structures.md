# Various important data structures

{{#title Important data structures}}

We have talked quite a lot about the `Response` and the `Request` structs, but of course this crates contains more than just these two structs. Let's take a closer look at what else is provided. It should be noted however that not all of the items here might be useful for the average library user; thischapter is meant to act more as a part of the documentation rather than a part of this tutorial book

## Structs

### `Target`

One of the most useful structs on this crate. Represents a HTTP URL/Target and also contains any HTTP queries passed in the URL.

- Fields

  - `target_path`: A `String` contains the path of the current handler (the string the user passes into the handler attach functions of the `Server` struct. For example, if we attach a handler into a `X` path, that would be the `X` string). Usually empty, except when the handler variant is set to `Directory`
  - `relative_path`: A `String` containing the rest of the URL, excluding `target_path`
  - `queries`: A `HashMap` of `String` to `String`. Each key-value pair represents the name and the value of a HTTP query

- Methods

  - `full_url()`: Appends `relative_path` into `target_path`, returns the result as a `String`

- Note

  - In order to get the original HTTP URL, use the `to_string()` method (implemented with the `Display` trait)

### `Version`

Represents the HTTP version of a `Request` or `Response`.

- Fields

  - `major`: A `usize` representing the major revision number of the HTTP version. The major revision number signifies significant updates and changes to the HTTP protocol.
  - `minor`: A `usize` representing the minor revision number of the HTTP version. The minor revision number indicates smaller updates and improvements made to the HTTP protocol without introducing major changes.

## Enums

### `Method`

The HTTP method of the HTTP `Request`

- Variants

  - `GET`: The `GET` method requests a representation of the specified resource. Requests using `GET` should only retrieve data.

  - `HEAD`: The `HEAD` method asks for a response identical to a the response to a `GET` request, but without the response body.

  - `POST`: The `POST` method submits an entity to the specified resource, often causing a change in state or side effects on the server.

  - `PUT`: The `PUT` method replaces all current representations of the target resource with the request payload.

  - `DELETE`: The `DELETE` method deletes the specified resource.

- Note

  - This enum is marked as non-exhaustive, which means that it could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm (`_ => { ... }`) must be added to account for any future variants.

### `HandlerMethod`

Extends the `Method` enum. Indicates when a handler should be called

- Variants

  - `Directory`: Represents a directory handler. It is the last handler type in terms of execution priority (that means that it will be called only when there is no other handler for a `Request`). Allows for the existance of more complicated handlers, such as file and directory handlers (as the name suggests). For example, if a `Directory` handler is attached to the `X` path, any `Request` whose `Target` URL begins with `X` will be executed by the handler
  - `Specific(Method)`: The handler will called when a `Request` with a `Specific` HTTP `Method` is made at the target path (`GET`, `POST`, etc.)
  - `Any`: The handler will be called on any `Request` under the specified path

### `Status`

The HTTP status of a `Response`

- Variants

  - `OK`: 200 OK
  - `Created`: 201 Created
  - `Accepted`: 202 Accepted
  - `NoContent`: 203 No Content
  - `BadRequest`: 400 Bad Request
  - `NotFound`: 404 Not Found
  - `InternalError`: 500 Internal Server Error
  - `NotImplemented`: 501 Not Implemented

- Note

  - This enum is marked as non-exhaustive, which means that it could have additional variants added in future. Therefore, when matching against variants of non-exhaustive enums, an extra wildcard arm (`_ => { ... }`) must be added to account for any future variants.

## Type Definitions

### `Handler`

Represents a `Request` handler

### `HandlerCallback`

The type of the function that processes a HTTP `Request` passed into a `Handler`
