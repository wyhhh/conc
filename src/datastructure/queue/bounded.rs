use super::RecvErr;
use super::SendErr;
use parking_lot::Condvar;
use parking_lot::Mutex;
use std::time::Duration;

#[derive(Debug)]
pub struct BoundedQ<T> {
    m: Mutex<Vec<T>>,
    c: Condvar,
    init_cap: usize,
}

impl<T> BoundedQ<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            m: Mutex::new(Vec::with_capacity(cap)),
            c: Condvar::new(),
            init_cap: cap,
        }
    }

    pub fn push(&self, ele: T) {
        unsafe { self.push_timeout(ele, Duration::MAX).unwrap_unchecked() }
    }

    pub fn push_timeout(&self, ele: T, d: Duration) -> Result<(), SendErr<T>> {
        let mut q = self.m.lock();

        // here we must use while avoiding other enqueue
        // got lock BUT the queue is full
        while q.len() == self.init_cap {
            if self.c.wait_for(&mut q, d).timed_out() {
                return Err(SendErr::Timeout(ele));
            }
        }

        debug_assert!(q.len() < self.init_cap);

        q.push(ele);

        // dropped lock
        drop(q);
        // because the q has just two state:
        // => FULL, n enqueue threads are blocking maybe;
        // => NON-FULL, m dequeue threads are blocking maybe
        // but, if when at FULL state, the one of dequeue thread
        // wakeup ALL the other threads, so the all BLOCKING
        // enqueue threads are awaken. thus, when we call
        // enqueue, there are likely some threads DEQUEUE blocking,
        // we just notify one of it will be OK.
        self.c.notify_one();
        Ok(())
    }

    pub fn pop(&self) -> T {
        unsafe { self.pop_timeout(Duration::MAX).unwrap_unchecked() }
    }

    pub fn pop_timeout(&self, d: Duration) -> Result<T, RecvErr> {
        let mut q = self.m.lock();

        // N.B. here must use whlie loop
        // because when a dequeue thread awaken
        // all the other threads including this
        // had been awaken, but it is empty, so
        // next line would panic.
        let ret = loop {
            match q.pop() {
                Some(ret) => break ret,
                None => {
                    if self.c.wait_for(&mut q, d).timed_out() {
                        return Err(RecvErr::Timeout);
                    }
                }
            }
        };
        drop(q);

        // here we need notify all
        self.c.notify_all();

        Ok(ret)
    }

    pub fn len(&self) -> usize {
        self.m.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Default for BoundedQ<T> {
    fn default() -> Self {
        Self::new(20)
    }
}

#[test]
fn test() {
    use rand::thread_rng;
    use rand::Rng;

    let mut assert_size = 0;
    let q = &BoundedQ::new(20);

    crossbeam_utils::thread::scope(|s| {
        for (i, enqueue, n) in (1..=201).map(|x| {
            let r = thread_rng().gen_range(-100..100_i32);
            (x, x % 2 == 1, r)
        }) {
            if enqueue {
                assert_size += 1;
            } else {
                assert_size -= 1;
            }

            println!(
                "{i}. [[ {} ]] => [ {} ]",
                if enqueue { "ENQUEUE" } else { "DEQUEUE" },
                if enqueue {
                    n.to_string()
                } else {
                    "".to_string()
                },
            );

            s.spawn(move |_| {
                if enqueue {
                    q.push(n);
                } else {
                    let ele = q.pop();
                    println!("dequeue element: {ele}");
                }
            });
        }
    })
    .unwrap();

    println!("////////// END SIZE: {:?} /////////", q.len());
    assert_eq!(assert_size, q.len());
}
