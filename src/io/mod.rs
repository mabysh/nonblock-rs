//! I/O primitives that use NonBlockingReader as their input and implement [`Future`] or
//! [`Stream`] traits. Intended to be used with [`tokio`] reactor.
//!
//![`Future`]: https://docs.rs/futures/0.2.0-alpha/futures/prelude/trait.Future.html
//![`Stream`]: https://docs.rs/futures/0.2.0-alpha/futures/prelude/trait.Stream.html
//![`tokio`]: https://tokio-rs.github.io/tokio/tokio/reactor/index.html

mod input_stream;

pub use self::input_stream::{input_stream, InputStream};
