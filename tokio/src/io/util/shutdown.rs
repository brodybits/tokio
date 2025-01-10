use crate::io::AsyncWrite;

use pin_project_lite::pin_project;
use core::future::Future;
// use std::io;
use core::marker::PhantomPinned;
use core::pin::Pin;
use core::task::{Context, Poll};

use portable_io as io;

pin_project! {
    /// A future used to shutdown an I/O object.
    ///
    /// Created by the [`AsyncWriteExt::shutdown`][shutdown] function.
    /// [shutdown]: [`crate::io::AsyncWriteExt::shutdown`]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    #[derive(Debug)]
    pub struct Shutdown<'a, A: ?Sized> {
        a: &'a mut A,
        // Make this future `!Unpin` for compatibility with async trait methods.
        #[pin]
        _pin: PhantomPinned,
    }
}

/// Creates a future which will shutdown an I/O object.
pub(super) fn shutdown<A>(a: &mut A) -> Shutdown<'_, A>
where
    A: AsyncWrite + Unpin + ?Sized,
{
    Shutdown {
        a,
        _pin: PhantomPinned,
    }
}

impl<A> Future for Shutdown<'_, A>
where
    A: AsyncWrite + Unpin + ?Sized,
{
    type Output = io::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.project();
        Pin::new(me.a).poll_shutdown(cx)
    }
}
