use std::time::Duration;

pub mod crossbeam_mpmc;
pub mod mpmc;
pub mod mpsc;

pub trait MpmcChannel<T> {
    type S: Sender<T> + Clone;
    type R: Receiver<T> + Clone;

    fn bounded(cap: usize) -> (Self::S, Self::R);
    fn unbounded() -> (Self::S, Self::R);
}

pub trait Sender<T> {
    type SendErr;
    type SendTimeoutErr;

    fn send(&self, msg: T) -> Result<(), Self::SendErr>;
    fn send_timeout(&self, msg: T, d: Duration) -> Result<(), Self::SendTimeoutErr>;
}

pub trait Receiver<T> {
    type RecvErr;
    type RecvTimeoutErr;

    fn recv(&self) -> Result<T, Self::RecvErr>;
    fn recv_timeout(&self, d: Duration) -> Result<T, Self::RecvTimeoutErr>;
}

pub trait Len {
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}