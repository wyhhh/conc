use super::MpmcChannel;
use super::Receiver;
use super::Sender;
use crossbeam_channel::RecvError;
use crossbeam_channel::RecvTimeoutError;
use crossbeam_channel::SendError;
use crossbeam_channel::SendTimeoutError;
use std::marker::PhantomData;
use std::time::Duration;
use super::Len;

impl<T> Sender<T> for crossbeam_channel::Sender<T> {
    type SendErr = SendError<T>;
    type SendTimeoutErr = SendTimeoutError<T>;

    fn send(&self, msg: T) -> Result<(), Self::SendErr> {
        self.send(msg)
    }
    fn send_timeout(&self, msg: T, d: Duration) -> Result<(), Self::SendTimeoutErr> {
        self.send_timeout(msg, d)
    }
}

impl<T> Receiver<T> for crossbeam_channel::Receiver<T> {
    type RecvErr = RecvError;
    type RecvTimeoutErr = RecvTimeoutError;

    fn recv(&self) -> Result<T, Self::RecvErr> {
        self.recv()
    }
    fn recv_timeout(&self, d: Duration) -> Result<T, Self::RecvTimeoutErr> {
        self.recv_timeout(d)
    }
}

#[derive(Debug, Default)]
pub struct CrossbeamChannel<T>(PhantomData<T>);

impl<T> MpmcChannel<T> for CrossbeamChannel<T> {
    type S = crossbeam_channel::Sender<T>;

    type R = crossbeam_channel::Receiver<T>;

    fn bounded(cap: usize) -> (Self::S, Self::R) {
        crossbeam_channel::bounded(cap)
    }

    fn unbounded() -> (Self::S, Self::R) {
        crossbeam_channel::unbounded()
    }
}

impl<T> Len for crossbeam_channel::Sender<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T> Len for crossbeam_channel::Receiver<T> {
    fn len(&self) -> usize {
        self.len()
    }
}