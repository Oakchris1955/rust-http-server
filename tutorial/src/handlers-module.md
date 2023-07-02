# Handlers module

{{#title Handlers module}}

We have already seen how a user can write it's own request handlers, but this doesn't mean that the crate doesn't come with some prewritten handlers.

## `read_same_dir`

Let's say that you want the server to serve some statis files, like images, HTML files or god knows what. Well, that's what this function essentially does: reads from the same directory as the path of the URL and sends them to the client.

### Usage

Simply pass the `read_same_dir` into your desired handler append function

### Example

```rust, no_run
{{#rustdoc_include code-examples/same_dir-example.rs:2:}}
```

In the above example, if the user requests a target at `/www/example.txt`, the server will send back the file located at `./www/example.txt` if it exists, otherwise it will respond with a `404 Not Found` error or `500 Internal Server Error` if the file exists and can't be opened

## `read_diff_dir()`

Same as above, but reads files from a directory different than the target directory.

### Usage

Pass the `read_diff_dir(READ_PATH)` into your desired handler append function, and substitute `READ_PATH` with the path from which you want the files to be read

### Example

```rust, no_run
{{#rustdoc_include code-examples/diff_dir-example.rs:2:}}
```

In the above example, if the user requests a target at `/www/example.txt`, the server will send back the file located at `./diff/example.txt` if it exists, otherwise it will respond with a `404 Not Found` error or `500 Internal Server Error` if the file exists and can't be opened
