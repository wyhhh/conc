use try_lock::TryLock;


type Link<T> = Option<Box<Node<T>>>;
pub struct SortedList<T> {
	len: u32, 
	head: Link<T>,
}

struct Node<T> {
	val: T,
	prev: Link<T>,
	next: Link<T>,
}