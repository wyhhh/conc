// use std::sync::atomic::AtomicU32;
// use std::sync::atomic::Ordering;
// use try_lock::TryLock;
// use try_lock::Locked;

// type Ptr<T> = TryLock<(usize, Option<Box<Node<T>>>)>;

// /// A hand over hand locking linked-list
// /// 
// pub struct List<T> {
//     head: Ptr<T>,
//     tail: Ptr<T>,
// }

// struct Node<T> {
//     prev: Ptr<T>,
//     next: Ptr<T>,
//     val: T,
// }

// impl<T> List<T> {
//     pub fn new() -> Self {
//         Self {
//             head: TryLock::new((0, None)),
//             tail: TryLock::new((0, None)),
//         }
//     }

//     pub fn len(&self) -> u32 {
		
//     }

//     pub fn is_empty(&self) -> bool {
//         self.len() == 0
//     }

//     pub fn push(&self, val: T) {
		
// 	}

// }

// #[cfg(test)]
// mod tests {
//     use crate::List;
//     #[test]
//     fn new() {
//         let l: List<i32> = List::new();

//         assert!(l.is_empty());
// 		assert!(matches!(*l.head().unwrap(), None));
//     }

// 	#[test]
// 	fn push() {

// 	}
// }
