pub mod bounded;
pub mod unbounded;

#[derive(Debug)]
pub enum RecvErr {
    Timeout,
}

#[derive(Debug)]
pub enum SendErr<T> {
    Timeout(T),
}

