This is an implementation of the minimal hot reloading described in https://robert.kra.hn/posts/hot-reloading-rust

You need to have Cargo Watch installed. https://crates.io/crates/cargo-watch

To run it:

```
python hot_run.py
```

The python script is a wrapper around `cargo run`, but also sets up a `cargo
watch` in the background that will rebuild the hot reloaded library when sources change.

Or, in a unix like shell you can run:
```
(trap 'kill 0' SIGINT; cargo watch -w lib -x 'build -p lib' & cargo run)
```

Which will run the `watch` and the `run` in a background shell
