# Minimal server example

Once you have successfully setup everything, it is time to create an example

## Editing main.rs

Let's begin by editing the `main.rs` file, which is from where our project's code is executed

Initially, `main.rs` should look like this

```rust
{{#rustdoc_include code-examples/initial.rs}}
```

Let's begin by including the library on the top of our `main.rs` file

```rust, no_run
{{#rustdoc_include code-examples/minimal-server.rs:2}}
```

Next, create a new `Server` instance:

```rust, no_run
{{#rustdoc_include code-examples/minimal-server.rs:5:8}}
```

Add a basic handler that always returns a static response:

```rust, no_run
{{#rustdoc_include code-examples/minimal-server.rs:10:12}}
```

Lastly, start the HTTP server:

```rust, no_run
{{#rustdoc_include code-examples/minimal-server.rs:14:16}}
```

Now, if you open your web browser and navigate to `localhost:2300/hello`, you should see a webpage displaying your message:
![miimal-server-view](img/minimal-server-browser-view.png)
