//! Read available data from file descriptors without blocking
//!
//! Useful for nonblocking reads from sockets, named pipes, and child stdout/stderr
//!
//! # Example
//!
//! ```no_run
//! use std::io::Read;
//! use std::process::{Command, Stdio};
//! use std::time::Duration;
//! use nonblock::NonBlockingReader;
//!
//! let mut child = Command::new("some-executable")
//!                         .stdout(Stdio::piped())
//!                         .spawn().unwrap();
//! let stdout = child.stdout.take().unwrap();
//! let mut noblock_stdout = NonBlockingReader::from_fd(stdout).unwrap();
//! while !noblock_stdout.is_eof() {
//!     let mut buf = String::new();
//!     noblock_stdout.read_available_to_string(&mut buf).unwrap();
//!     std::thread::sleep(Duration::from_secs(5));
//! }
//! ```
extern crate libc;
extern crate mio;
extern crate tokio;
#[macro_use]
extern crate tokio_io;
extern crate bytes;
extern crate futures;

mod reader;
mod io;

pub use reader::NonBlockingReader;
pub use io::{input_stream, InputStream};
