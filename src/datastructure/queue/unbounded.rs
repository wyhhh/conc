use super::RecvErr;
use parking_lot::Condvar;
use parking_lot::Mutex;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct UnboundedQ<T> {
    m: Mutex<Vec<T>>,
    c: Condvar,
}

impl<T> UnboundedQ<T> {
    pub fn new() -> Self {
        Self::with_capacity(5)
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self {
            m: Mutex::new(Vec::with_capacity(cap)),
            c: Condvar::new(),
        }
    }

    pub fn push(&self, ele: T) {
        let mut g = self.m.lock();
        g.push(ele);

        if g.len() == 1 {
            self.c.notify_one();
        }
    }

    /// Block pop util it has element
    pub fn pop(&self) -> T {
        unsafe { self.pop_timeout(Duration::MAX).unwrap_unchecked() }
    }

    /// Block pop but util timeout
    /// If timeout, return Err(Timerout), otherwise Ok(T)
    pub fn pop_timeout(&self, d: Duration) -> Result<T, RecvErr> {
        let mut g = self.m.lock();

        loop {
            match g.pop() {
                Some(ret) => return Ok(ret),
                None => {
                    if self.c.wait_for(&mut g, d).timed_out() {
                        return Err(RecvErr::Timeout);
                    }
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.m.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod test {
    use super::UnboundedQ;
    use std::thread;
    use std::time::Duration;
    #[test]
    fn test_timeout() {
        let q = &UnboundedQ::<i32>::new();

        crossbeam_utils::thread::scope(|s| {
            s.spawn(move |_| {
                println!("{:?}", q.pop_timeout(Duration::from_secs(1)));
            });
            s.spawn(move |_| {
                println!("{:?}", q);
            });
        })
        .unwrap();
    }

    #[test]
    fn test_blocking() {
        let q = &UnboundedQ::<i32>::new();

        crossbeam_utils::thread::scope(|s| {
            s.spawn(move |_| {
                thread::sleep(Duration::from_secs(2));
                q.push(1);
                // q.push(3);
                // q.push(4);
                // q.push(5);
            });
            s.spawn(move |_| {
                println!("{:?}", q.pop());
                println!("{:?}", q.len());
                // blocking from here
                println!("{:?}", q.pop());
            });
        })
        .unwrap();
    }
}
