macro_rules! ready {
    ($e:expr) => {
        match $e {
            std::task::Poll::Ready(v) => v,
            std::task::Poll::Pending => return std::task::Poll::Pending,
        }
    };
}

pub mod buf;
#[cfg(all(feature = "server", any(feature = "http1", feature = "http2")))]
pub mod date;
#[cfg(all(feature = "server", any(feature = "http1", feature = "http2")))]
pub mod drain;
#[cfg(any(feature = "http1", feature = "http2"))]
pub mod exec;
pub mod io;
#[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
mod lazy;
mod never;
#[cfg(feature = "stream")]
pub mod sync_wrapper;
pub mod task;
pub mod watch;

#[cfg(all(feature = "client", any(feature = "http1", feature = "http2")))]
pub(crate) use self::lazy::{lazy, Started as Lazy};
#[cfg(any(
    feature = "client",
    feature = "http1",
    feature = "http2",
    feature = "runtime"
))]
pub(crate) use self::never::Never;
pub(crate) use self::task::Poll;

// group up types normally needed for `Future`
cfg_proto! {
    pub(crate) use std::marker::Unpin;
}
pub(crate) use std::{future::Future, pin::Pin};
