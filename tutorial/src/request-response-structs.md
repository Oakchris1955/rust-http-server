# The Request and Response structs

In this chapter, we will learn how to use the `Request` and `Response` structs, which are passed to each handler as the first and second argument respectively.

## Some generic info

Before we move on, we should probably clarify a difference between the two structs:

- The `Request` struct is basically a read-only struct: that means that the library fills out the struct's field for the program to read from them and process them accordingly

- On the other hand, the `Response` struct is basically a write-only struct, which means that the library provides many methods for the program to write data to the HTTP connection but the struct itself doesn't contain any useful data by default.

Furthermore, both structs contain the following fields:

- `version`: The HTTP version of the `Request`/`Response`. A simple struct with two fields: `major` and `minor`.
- `headers`: A type alias that resolves to a `HashMap` of `String` to `String` that contains the `Request`/`Response` headers. For each pair, the key is the header name and the value the header value

## The Request struct

The `Request` struct contains info about the request made by the client.

It contains the following fields:

- `method`: A `Method` enum that represents the HTTP method of this request (pretty useful if you have a generic handler and the logic of your handler depends on the HTTP method of the request)
- `target`: A `Target` struct representing the target URL of the `Request`. More info about this on a seperate chapter
- `body`: A `Vec<u8>` containing the request body as a byte vector (most requests don't have a body, some of them do however, for example when the MIME type is `application/x-www-form-urlencoded`)
- `cookies`: `Cookies` type alias which resolves to a `HashMap` of `String` to `String`. While the program can also access the request cookies through the `Cookie` header, it is probably more convenient to access them through a specialized struct field, such as this

## The Response struct

The `Response` struct contains info about the response to send to the client, alongside with some methods to actually send the response.

It contains the following field:

- `status`: A `Status` enum that represents the HTTP response status

It also contains the following methods:

- `status`: Changes the `Response`'s `status` field. Takes a single parameter, the `Status` to which to set the `status` field of the `Response`
- `set_cookie`: Set a new `Cookie` (will be covered in the next chapter) to send alongside this response. Takes a single parameter, the `Cookie` to append to the internal `cookies` field
- `set_header`: Set a new HTTP header to send with this response. Takes two parameters, the name and the value of the HTTP header to set (both parameters must implement the `ToString` trait)
- `set_headers`: Set multiple HTTP headers at once. Takes a single parameter, the `Headers` type alias covered above
- `get_headers`: Get an unmutable reference (`&Headers`) to the internal headers fields.
- `send`[*]: Send some data to the client. Takes a single argument, the data to send to the client. Can be called multiple times in a row
- `end`[*]: End the response by consuming it
- `end_with`[*]: Send some data, then end the response (calls `send()`, then `end()`)

[*]: http://localhost:3000/request-handlers.html#about-error-handling
