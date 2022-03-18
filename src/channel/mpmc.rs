use super::Len;
use super::MpmcChannel;
use crate::datastructure::queue::RecvErr;
use crate::datastructure::queue::SendErr;
use crate::BoundedQ;
use crate::UnboundedQ;
use std::marker::PhantomData;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct DefaultChannel<T>(PhantomData<T>);

impl<T> MpmcChannel<T> for DefaultChannel<T> {
    type S = Sender<T>;

    type R = Receiver<T>;

    fn bounded(cap: usize) -> (Self::S, Self::R) {
        let q = Arc::new(BoundedQ::new(cap));
        (
            Sender(Inner::Bounded(q.clone())),
            Receiver(Inner::Bounded(q)),
        )
    }

    fn unbounded() -> (Self::S, Self::R) {
        let q = Arc::new(UnboundedQ::new());
        (
            Sender(Inner::Unbounded(q.clone())),
            Receiver(Inner::Unbounded(q)),
        )
    }
}
#[derive(Debug)]
pub struct Sender<T>(Inner<T>);

#[derive(Debug)]
pub struct Receiver<T>(Inner<T>);

#[derive(Debug)]
enum Inner<T> {
    Bounded(Arc<BoundedQ<T>>),
    Unbounded(Arc<UnboundedQ<T>>),
}

impl<T> super::Sender<T> for Sender<T> {
    type SendErr = SendErr<T>;
    type SendTimeoutErr = SendErr<T>;

    fn send(&self, msg: T) -> Result<(), Self::SendErr> {
        self.0.send(msg)
    }

    fn send_timeout(&self, msg: T, d: Duration) -> Result<(), Self::SendTimeoutErr> {
        self.0.send_timeout(msg, d)
    }
}

impl<T> super::Receiver<T> for Receiver<T> {
    type RecvErr = RecvErr;
    type RecvTimeoutErr = RecvErr;

    fn recv(&self) -> Result<T, Self::RecvErr> {
        self.0.recv()
    }
    fn recv_timeout(&self, d: Duration) -> Result<T, Self::RecvTimeoutErr> {
        self.0.recv_timeout(d)
    }
}

impl<T> Len for Sender<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> Len for Receiver<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}


impl<T> Inner<T> {
    fn send(&self, ele: T) -> Result<(), SendErr<T>> {
        Ok(match self {
            Inner::Bounded(b) => b.push(ele),
            Inner::Unbounded(u) => u.push(ele),
        })
    }

    fn send_timeout(&self, ele: T, d: Duration) -> Result<(), SendErr<T>> {
        match self {
            Inner::Bounded(b) => b.push_timeout(ele, d),
            Inner::Unbounded(u) => Ok(u.push(ele)),
        }
    }

    fn recv(&self) -> Result<T, RecvErr> {
        Ok(match self {
            Inner::Bounded(b) => b.pop(),
            Inner::Unbounded(u) => u.pop(),
        })
    }

    fn recv_timeout(&self, d: Duration) -> Result<T, RecvErr> {
        match self {
            Inner::Bounded(b) => b.pop_timeout(d),
            Inner::Unbounded(u) => u.pop_timeout(d),
        }
    }

    fn len(&self) -> usize {
        match self {
            Inner::Bounded(b) => b.len(),
            Inner::Unbounded(u) => u.len(),
        }
    }
}

impl<T> Clone for Inner<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Bounded(arg0) => Self::Bounded(arg0.clone()),
            Self::Unbounded(arg0) => Self::Unbounded(arg0.clone()),
        }
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Clone for Receiver<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(test)]
mod test {
    use super::DefaultChannel;
    use crate::channel::MpmcChannel;
    use crate::channel::Receiver;
    use crate::channel::Sender;
    use crate::channel::Len;

    #[test]
    fn test() {
        let (tx, rx) = DefaultChannel::unbounded();

        crossbeam_utils::thread::scope(|s| {
            for n in 0..10 {
                let tx = tx.clone();

                s.spawn(move |_| {
                    tx.send(n);
					println!("{:?}",tx.len());
                });

                for n in 0..2 {
                    let rx = rx.clone();

                    s.spawn(move |_| {
                        let res = rx.recv();
                        println!("{:?}", res);
                    });
                }
            }
        })
        .unwrap();
    }
}
