pub mod queue;

pub trait BoundedBlockQueue<T> {
    fn new(cap: usize) -> Self;
    fn enqueue(&self, ele: T);
    fn dequeue(&self) -> T;
    fn len(&self) -> usize;
}
