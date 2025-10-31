extern crate alloc;
use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};
use alloc::boxed::Box;

pub struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

