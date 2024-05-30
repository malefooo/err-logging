
use pin_project::pin_project;
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

thread_local! {
    static ASYNC_THREAD_UUID: RefCell<String> = RefCell::new(String::new());
}

pub fn async_get() -> String {
    ASYNC_THREAD_UUID.with(|uid| uid.borrow().clone())
}

#[pin_project]
struct AsyncUid<Fut> {
    #[pin]
    future: Fut,
    uid: String,
}

impl<Fut> Future for AsyncUid<Fut>
where
    Fut: Future,
{
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let uid = this.uid.clone();

        ASYNC_THREAD_UUID.set(uid);

        let mut future = Box::pin(async move { this.future.await });
        Pin::new(&mut future).poll(cx)
    }
}

#[allow(dead_code)]
trait FutureExt: Future {
    fn with_uid(self, uid: String) -> AsyncUid<Self>
    where
        Self: Sized,
    {
        // ASYNC_THREAD_UUID.scope()
        AsyncUid { future: self, uid }
    }
}

impl<F: Future> FutureExt for F {}
