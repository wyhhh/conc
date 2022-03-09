use parking_lot::Condvar;
use parking_lot::Mutex;

#[derive(Default, Debug)]
pub struct Sema {
    m: Mutex<i64>,
    cv: Condvar,
    origin: i64,
}

impl Sema {
    pub fn new(permits: i64) -> Self {
        Self {
            m: Mutex::new(permits),
            cv: Condvar::new(),
            // When new a sema, normally at
            // one thread without sharing.
            // This one can be runtime value
            // but never changed again
            // so we needn't add sync method
            // on it
            origin: permits,
        }
    }

    /// If `permits` is negative, it will call
    /// [`Sema::acquiren`], otherwise positive, call
    /// [`Sema::releasen`]. Zero does nothing.
    #[allow(clippy::comparison_chain)]
    pub fn set(&self, permits: i32) {
        if permits < 0 {
            self.acquiren((-permits) as u32);
        } else if permits > 0 {
            self.releasen(permits as u32);
        }
    }

    pub fn acquiren(&self, permits: u32) {
        let permits = permits as i64;
        let mut g = self.m.lock();

        while *g < permits {
            self.cv.wait(&mut g);
        }

        *g -= permits;
    }

    pub fn releasen(&self, permits: u32) {
        {
            let mut g = self.m.lock();
            *g += permits as i64;

            if *g <= 0 {
                return;
            }
        }

        self.cv.notify_all();
    }

    pub fn acquire(&self) {
        self.acquiren(1);
    }

    pub fn release(&self) {
        self.releasen(1);
    }

    pub fn available(&self) -> i64 {
        *self.m.lock()
    }

    /// Reset permits to original one
    // Because the origin value can't
    // changed from first initialization
    // so we needn't use like AtomicI64
    pub fn reset(&self) {
        *self.m.lock() = self.origin;
    }

    /// Reset permits at runtime once
    pub fn reset_permits(&self, permits: i64) {
        *self.m.lock() = permits;
    }
}
