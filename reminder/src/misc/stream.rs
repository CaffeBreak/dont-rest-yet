use std::{
    ops::Deref,
    task::{Context, Poll},
};

use tokio::{
    sync::{mpsc, oneshot},
    task::JoinHandle,
};
use tokio_stream::Stream;

use crate::log;

pub(crate) struct DropReceiver<T> {
    pub(crate) chan: Option<oneshot::Sender<usize>>,
    pub(crate) inner: mpsc::Receiver<T>,
    pub(crate) join_handle: JoinHandle<()>,
}

impl<T> Stream for DropReceiver<T> {
    type Item = T;

    fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<T>> {
        self.inner.poll_recv(cx)
    }
}

impl<T> Deref for DropReceiver<T> {
    type Target = mpsc::Receiver<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> Drop for DropReceiver<T> {
    fn drop(&mut self) {
        log!("DEBUG" -> "Receiver has been dropped".dimmed());
        if let Some(sender) = self.chan.take() {
            if sender.send(1).is_err() {
                self.inner.close();
                self.join_handle.abort();

                log!("gRPC" -> format!(">>> Close push notification stream.").cyan());
            }
        }
    }
}
