//! Ref: <https://github.com/dpruessner/axum-static-s3/blob/main/src/adapter.rs>

use std::{
    io::Error,
    pin::Pin,
    task::{Context, Poll},
};

use futures_core::Stream;
use pin_project::pin_project;
use tokio::io::{AsyncRead, ReadBuf};

#[pin_project]
pub struct StreamAdapter<T> {
    #[pin]
    stream: T,
}

impl<T: AsyncRead> StreamAdapter<T> {
    pub fn new(stream: T) -> Self {
        Self { stream }
    }
}

impl<T: AsyncRead> Stream for StreamAdapter<T> {
    type Item = Result<Vec<u8>, Error>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut buf = [0; 1024];
        let mut read_buf = ReadBuf::new(&mut buf);

        let this = self.project();
        let stream = this.stream;

        match stream.poll_read(cx, &mut read_buf) {
            Poll::Ready(Ok(())) => {
                let n = read_buf.filled().len();
                if n > 0 {
                    Poll::Ready(Some(Ok(buf[..n].to_vec())))
                } else {
                    Poll::Ready(None)
                }
            }
            Poll::Ready(Err(e)) => Poll::Ready(Some(Err(e))),
            Poll::Pending => Poll::Pending,
        }
    }
}
