use parking_lot::Condvar;
use parking_lot::Mutex;

pub struct Sema {
    m: Mutex<i64>,
    cv: Condvar,
}

impl Sema {
    pub fn new(permits: i64) -> Self {
        Self {
            m: Mutex::new(permits),
            cv: Condvar::new(),
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
}
