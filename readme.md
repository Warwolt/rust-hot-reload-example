This is an implementation of the minimal hot reloading described in https://robert.kra.hn/posts/hot-reloading-rust

You need to have Cargo Watch installed. https://crates.io/crates/cargo-watch

To run it:

```
cargo watch -w lib -x 'build -p lib'
```
