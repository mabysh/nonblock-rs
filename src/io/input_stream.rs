use std::io;
use std::io::Read;
use std::os::unix::io::AsRawFd;
use std::mem;

use futures;
use futures::Stream;

use tokio::prelude::*;
use tokio::reactor::PollEvented2;

use super::super::reader::NonBlockingReader;

/// A stream that yeilds bytes without blocking as soon as they appear on reader.
/// Created with [`input_stream`] function.
///
/// [`input_stream`]: fn.input_stream.html
#[derive(Debug)]
pub struct InputStream<R>
where
    R: AsRawFd + Read,
{
    io: PollEvented2<NonBlockingReader<R>>,
    cap: Option<usize>,
    buf: Vec<u8>,
}

/// Create a stream of bytes to read from file descriptor until EOF is reached.
/// It is assumed that file desctriptor can be set into nonblocking mode.
pub fn input_stream<R: AsRawFd + Read>(fd: R, buf_capacity: Option<usize>) -> InputStream<R> {
    let reader =
        NonBlockingReader::from_fd(fd).expect("Failed to set O_NONBLOCK to the file descriptor");
    InputStream {
        io: PollEvented2::new(reader),
        cap: buf_capacity,
        buf: new_vec(buf_capacity),
    }
}

impl<R: AsRawFd + Read> Stream for InputStream<R> {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn poll(&mut self) -> futures::Poll<Option<Vec<u8>>, io::Error> {
        match try_nb!(self.io.read_buf(&mut self.buf)) {
            futures::Async::Ready(_i) => {
                if self.io.get_ref().is_eof() {
                    // EOF
                    return Ok(None.into());
                }
                let len = self.buf.len();
                if len >= 1 && self.buf[len - 1] == b'\n' {
                    self.buf.pop();
                    if len >= 2 && self.buf[len - 2] == b'\r' {
                        self.buf.pop();
                    }
                }
                return Ok(Some(mem::replace(&mut self.buf, new_vec(self.cap))).into());
            }
            futures::Async::NotReady => {
                return Ok(Async::NotReady);
            }
        }
    }
}

fn new_vec(size: Option<usize>) -> Vec<u8> {
    match size {
        Some(s) => Vec::with_capacity(s),
        None => Vec::new(),
    }
}
