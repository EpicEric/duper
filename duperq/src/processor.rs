use async_trait::async_trait;
use duper::DuperValue;
use smol::{
    channel,
    io::{AsyncWrite, AsyncWriteExt},
};
use yoke::Yoke;

use crate::filter::DuperFilter;

#[async_trait(?Send)]
/// An opaque layer that processes a [`DuperValue`] asynchronously.
pub trait Processor {
    async fn process(&mut self, value: Yoke<DuperValue<'static>, String>);

    async fn close(&mut self) {}
}

pub(crate) struct FilterProcessor {
    filter: Box<dyn DuperFilter>,
    sender: channel::Sender<Yoke<DuperValue<'static>, String>>,
    is_open: bool,
}

impl FilterProcessor {
    pub(crate) fn new(
        sender: channel::Sender<Yoke<DuperValue<'static>, String>>,
        filter: Box<dyn DuperFilter>,
    ) -> Self {
        Self {
            is_open: true,
            sender,
            filter,
        }
    }
}

#[async_trait(?Send)]
impl Processor for FilterProcessor {
    async fn process(&mut self, value: Yoke<DuperValue<'static>, String>) {
        if self.is_open && self.filter.filter(value.get()) && self.sender.send(value).await.is_err()
        {
            self.is_open = false;
        }
    }
}

pub(crate) struct TakeProcessor {
    available: usize,
    sender: channel::Sender<Yoke<DuperValue<'static>, String>>,
}

impl TakeProcessor {
    pub(crate) fn new(
        sender: channel::Sender<Yoke<DuperValue<'static>, String>>,
        available: usize,
    ) -> Self {
        Self { sender, available }
    }
}

#[async_trait(?Send)]
impl Processor for TakeProcessor {
    async fn process(&mut self, value: Yoke<DuperValue<'static>, String>) {
        if self.available > 0 {
            if self.sender.send(value).await.is_err() {
                self.available = 0;
            } else {
                self.available = self.available.saturating_sub(1);
                if self.available == 0 {
                    self.sender.close();
                }
            }
        }
    }
}

pub(crate) struct SkipProcessor {
    to_skip: usize,
    sender: channel::Sender<Yoke<DuperValue<'static>, String>>,
    is_open: bool,
}

impl SkipProcessor {
    pub(crate) fn new(
        sender: channel::Sender<Yoke<DuperValue<'static>, String>>,
        to_skip: usize,
    ) -> Self {
        Self {
            sender,
            to_skip,
            is_open: true,
        }
    }
}

#[async_trait(?Send)]
impl Processor for SkipProcessor {
    async fn process(&mut self, value: Yoke<DuperValue<'static>, String>) {
        if self.to_skip > 0 {
            self.to_skip = self.to_skip.saturating_sub(1);
        } else if self.is_open && self.sender.send(value).await.is_err() {
            self.is_open = false;
        }
    }
}

pub(crate) struct OutputProcessor<O> {
    output: O,
    printer: Box<dyn FnMut(Yoke<DuperValue<'static>, String>) -> Vec<u8>>,
}

impl<O> OutputProcessor<O> {
    pub(crate) fn new(
        output: O,
        printer: Box<dyn FnMut(Yoke<DuperValue<'static>, String>) -> Vec<u8>>,
    ) -> Self {
        Self { output, printer }
    }
}

#[async_trait(?Send)]
impl<O> Processor for OutputProcessor<O>
where
    O: AsyncWrite + Unpin + 'static,
{
    async fn process(&mut self, value: Yoke<DuperValue<'static>, String>) {
        self.output
            .write_all((self.printer)(value).as_ref())
            .await
            .expect("stdout was closed");
        self.output
            .write_all(b"\n")
            .await
            .expect("stdout was closed");
    }

    async fn close(&mut self) {
        self.output.flush().await.expect("stdout was closed");
        self.output.close().await.expect("stdout was closed");
    }
}
