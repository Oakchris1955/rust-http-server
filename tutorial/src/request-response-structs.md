# The Request and Response structs

In the past chapter, we covered basically everything about request handlers and how to utilize them, but we still haven't learned what kind of data we can read from the `Request` and `Response` structs passed to each handler. Let's find out

## Some general info

Before we move on, we should probably clarify a difference between the two structs:

- The `Request` struct is basically a read-only struct: that means that the user should only care about the struct's fields and that not many methods are available to modify the struct.

- On the other hand, the `Response` struct is a write-only struct, which means that the user mainly calls methods from this struct and that most of the operations. can be done through methods

Furthermore, both structs contain the following fields:

- `version`: The HTTP version of the `Request`/`Response`. A simple struct with two fields: `major` and `minor`.
- `headers`: A `HashMap` of `String` to `String` that contains the `Request`/`Response` headers. For each pair, the key is the header name and the value the header value

## The Request struct

The `Request` struct contains info about the request made by the user.

It contains the following fields:

- `method`: A `Method` enum that represents the HTTP method of this request (pretty useful if you have a generic handler and the logic of your handler depends on the HTTP method of the request)
- `target`: A `Target` struct representing the target URL of the `Request`. More info about this on a seperate chapter

## The Response struct

The `Response` struct contains info about the response to send to the user, along with some methods to actually send the response.

It contains the following field:

- `status`: A `Status` enum that represents the HTTP response status

This struct also contains some methods:

- `status`: Changes the `Response`'s `status` field. Take a single parameter, the `Status` to which to set the `status` field of the `Response`
- `send`: Sends the `Response` to the user, while consuming the `Response` struct. Takes a single argument, which is the body content for the `Response`
- `end`: Same as `send`, but send a body content with zero (0) length
