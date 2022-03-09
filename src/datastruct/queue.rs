use crate::BoundedBlockQueue;
use parking_lot::Condvar;
use parking_lot::Mutex;

pub struct BoundedBlockQ<T> {
    m: Mutex<Vec<T>>,
    c: Condvar,
    init_cap: usize,
}

impl<T> BoundedBlockQueue<T> for BoundedBlockQ<T> {
    fn new(cap: usize) -> Self {
        Self {
            m: Mutex::new(Vec::with_capacity(cap)),
            c: Condvar::new(),
            init_cap: cap,
        }
    }

    fn enqueue(&self, ele: T) {
        {
            let mut q = self.m.lock();

            // here we must use while avoiding other enqueue
            // got lock BUT the queue is full
            while q.len() == self.init_cap {
                self.c.wait(&mut q);
            }

            debug_assert!(q.len() < self.init_cap);

            q.push(ele);

            // dropped lock
        }

        // because the q has just two state:
        // => FULL, n enqueue threads are blocking maybe;
        // => NON-FULL, m dequeue threads are blocking maybe
        // but, if when at FULL state, the one of dequeue thread
        // wakeup ALL the other threads, so the all BLOCKING
        // enqueue threads are awaken. thus, when we call
        // enqueue, there are likely some threads DEQUEUE blocking,
        // we just notify one of it will be OK.
        self.c.notify_one();
    }

    fn dequeue(&self) -> T {
        let ret = {
            let mut q = self.m.lock();

            // N.B. here must use whlie loop
            // because when a dequeue thread awaken
            // all the other threads including this
            // had been awaken, but it is empty, so
            // next line would panic.
            while q.is_empty() {
                self.c.wait(&mut q);
            }

            q.pop().unwrap()
            // unlock here
        };

        // here we need notify all
        self.c.notify_all();

        ret
    }

    fn len(&self) -> usize {
        self.m.lock().len()
    }
}

#[test]
fn test() {
    use rand::thread_rng;
    use rand::Rng;

    let mut assert_size = 0;
    let q = &BoundedBlockQ::new(20);

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
                    q.enqueue(n);
                } else {
                    let ele = q.dequeue();
                    println!("dequeue element: {ele}");
                }
            });
        }
    })
    .unwrap();

    println!("////////// END SIZE: {:?} /////////", q.len());
    assert_eq!(assert_size, q.len());
}
