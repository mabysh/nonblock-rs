# nonblock-rs
Read available data from file descriptors without blocking

This is fork of [original crate](https://github.com/anowell/nonblock-rs)

## Examples

See [structure-stdio.rs](examples/structure-stdio.rs/) for an example usage.

## Build & Test

This project is built and tested with cargo:

```bash
cargo build
cargo test
cargo doc --no-deps
```

Pro-tip: before building docs, clone existing docs to track changes
```bash
git clone -b gh-pages git@github.com:anowell/nonblock-rs.git target/doc
```

