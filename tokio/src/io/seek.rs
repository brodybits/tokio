use crate::io::AsyncSeek;

use pin_project_lite::pin_project;
use core::future::Future;
// use std::io::{self, SeekFrom};
use core::marker::PhantomPinned;
use core::pin::Pin;
use core::task::{ready, Context, Poll};

use portable_io as io;
use portable_io::SeekFrom;

pin_project! {
    /// Future for the [`seek`](crate::io::AsyncSeekExt::seek) method.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Seek<'a, S: ?Sized> {
        seek: &'a mut S,
        pos: Option<SeekFrom>,
        // Make this future `!Unpin` for compatibility with async trait methods.
        #[pin]
        _pin: PhantomPinned,
    }
}

pub(crate) fn seek<S>(seek: &mut S, pos: SeekFrom) -> Seek<'_, S>
where
    S: AsyncSeek + ?Sized + Unpin,
{
    Seek {
        seek,
        pos: Some(pos),
        _pin: PhantomPinned,
    }
}

impl<S> Future for Seek<'_, S>
where
    S: AsyncSeek + ?Sized + Unpin,
{
    type Output = io::Result<u64>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        match me.pos {
            Some(pos) => {
                // ensure no seek in progress
                ready!(Pin::new(&mut *me.seek).poll_complete(cx))?;
                match Pin::new(&mut *me.seek).start_seek(*pos) {
                    Ok(()) => {
                        *me.pos = None;
                        Pin::new(&mut *me.seek).poll_complete(cx)
                    }
                    Err(e) => Poll::Ready(Err(e)),
                }
            }
            None => Pin::new(&mut *me.seek).poll_complete(cx),
        }
    }
}
