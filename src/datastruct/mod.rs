pub mod queue;

pub trait BoundedBlockQueue<T> {
    fn new(cap: usize) -> Self;
    fn enqueue(&self, ele: T);
    fn dequeue(&self) -> T;
    fn len(&self) -> usize;
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}
