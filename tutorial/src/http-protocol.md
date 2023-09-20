# About the HTTP protocol

In order to understand some more complex aspects of this library, you must first understand the basics of the **HTTP protocol**

## What exactly is HTTP anyways?

HTTP stands for **HyperText Transfer Protocol** and is one of the most popular ways of **client-server communication** in the Internet. In HTTP, the server is listening for requests from clients, handles them accordingly and then sends a response back to the client.

Let's take a look at how a typical **HTTP exchange** looks like:

1) A connection is established between the client and the server on an underlying protocol, which is most frequently the **TCP** protocol
  
    It should be noted here that the HTTP specs don't specify what underlying protocol to use. However, out of the 2 most commonly-used protocols specified in the Internet protocol suite (also known as TCP/IP), TCP is the most suitable for client-server communications. The other candidate, UDP, is less reliable and doesn't guarantee that a message will arrive to its destination
2) The client begins sending data to the server in the form of a HTTP request. If it doesn't and stays idle, the server will most likely drop the connection after a specified timeout
3) Once the client is done sending data to the server, the server will then process this data and send a response back to the user
4) The connection may or may not be dropped then. This depends both on the client and the server. In case the connection isn't dropped, it could be used later on to exchange more data

In any case, if any request or response is found malformed, the server or the client respectively have the right to drop the connection immediately.

## HTTP messages

Both HTTP requests and responses follow the format of a HTTP message with some exceptions. Both of them contain:

- The HTTP version of the message
- Various headers
- Body message (it isn't necessary for a HTTP message to have one, most HTTP requests don't have a body)

Furthermore, a request has the following:

- A request method
- A target path

Whereas, a HTTP response has:

- A response status code
- A response status text describing what the corresponding status code means

## HTTP requests

A HTTP request is exactly what it says it is: a request towards the server to access/modify/submit some data. The most important fields are the method and the target fields.

### HTTP Methods

There are many HTTP request methods, the most common of which are:

- `GET`: Requests a representation of the specified resource
- `HEAD`: Asks for a response identical to a GET request, but without the response body
- `POST`: Submits an entity to the specified resource, often causing a change in state or side effects on the server.
- `PUT`: Same as `PUT`, except that calling it once or several times successively has the same effect (that is no side effect), where successive identical POST may have additional effects
- `DELETE`: Deletes the specified resource.

### HTTP target

The request target on the other hand is a string that represents the resource to access.

For example, making a `GET` request at a resource could be used return that resource, making a `DELETE` request could be used delete it. It is up to the server to decide how to implement each method. This library basically gives full control over the user on how to handle such cases

## HTTP responses

A HTTP response on the other hand is sent by the server in order to notify the client about the result of the request or/and send back some data. There is basically one field important here, which is the response status field

### HTTP status codes

The HTTP status codes are 3-digit number that are divided into 5 categories:

1) Informational responses (100 – 199)
2) Successful responses (200 – 299)
3) Redirection messages (300 – 399)
4) Client error responses (400 – 499)
5) Server error responses (500 – 599)

The most commonly known status codes are:

- `200 OK`: The request succeeded.
- `301 Moved Permanently`: The URL of the requested resource has been changed permanently. The new URL is given in the response. Most commonly used in redirects
- `308 Permanent Redirect`: Like `301`, but the user can't switch methods.
- `400 Bad Request`: The request is malformed and won't be processed. The client should not repeat this request without modification
- `401 Unauthorized`: The client must authenticate itself to get the requested response and make itself known to the server.
- `402 Forbidden`: Like `401`, but the server knows who the client is, the client just isn't authorized **enough** to access the resource.
- `404 Not Found`: The server cannot find the requested resource. In an API, this can also mean that the endpoint is valid but the resource itself does not exist. Sent by this library when no handlers are assigned to the requested target
- `429 Too Many Requests`: The user has sent too many requests in a given amount of time ("rate limiting").
- `500 Internal Server Error`: The server has encountered a situation it does not know how to handle. Usually indicates an in-server error.

### HTTP headers

Headers provide information about the message itself, it's encoding, the body length, even the time it was created.

There are many standarized HTTP headers. Some of them can be found at the [IANA registry](https://www.iana.org/assignments/message-headers/message-headers.xhtml)

## References

For more info, check the [MDN Docs](https://developer.mozilla.org/en-US/) and [this webpage](https://www.codetinkerer.com/2015/12/04/choosing-an-http-status-code.html) by [Michael Kropat](https://www.codetinkerer.com/about)
