# WebAssembly Site (with axum and yew)

This is the beginnings of an opinionated template to build web sites in WebAssembly, using the Yew framework. It has a backend server, leveraging axum, which serves up the site.

Inspiration for this template came from:

[A Rust web server/frontend setup like it's 2022](https://robert.kra.hn/posts/2022-04-03_rust-web-wasm/) by Robert Krahn

To work in dev mode, at a terminal prompt type:

```
cd web-wasm-project/frontend
trunk serve
```

At a new terminal prompt type:

```
cd web-wasm-project/server
cargo watch -- cargo run -- --port 8081
```

Go to http://localhost:8080 in the browser of your choice.

Hot reload is enabled, so you can make changes additions to the content and observer the result.

