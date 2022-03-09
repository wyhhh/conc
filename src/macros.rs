#[macro_export]
macro_rules! shared {
    ($($name: ident),+) => {$(
		#[derive(Default, Debug)]
		pub struct $name(std::sync::Arc<$crate::$name>);

		impl $name {
			pub fn new() -> Self {
				Default::default()
			}
		}

		impl std::ops::Deref for $name {
			type Target = std::sync::Arc<$crate::$name>;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}
	)+};
}

#[macro_export]
macro_rules! shared_generic {
    ($($name: ident),+) => {$(
		#[derive(Default, Debug)]
		pub struct $name<T>(std::sync::Arc<$crate::$name<T>>);

		impl<T: Default> $name<T> {
			pub fn new() -> Self {
				Default::default()
			}
		}

		impl<T> std::ops::Deref for $name<T> {
			type Target = std::sync::Arc<$crate::$name<T>>;

			fn deref(&self) -> &Self::Target {
				&self.0
			}
		}
	)+};
}

pub mod shared{
	shared!(Sema);
	shared_generic!(BoundedBlockQ);
}


#[test]
fn test() {
	use crate::BoundedBlockQueue;

	let q = shared::BoundedBlockQ::new();

	q.enqueue(2);

	let q2 = q.clone();

	println!("{:?}",q2);

	let s = shared::Sema::new();
	s.release();

	println!("{:?}",s);
	let _s2 = s.clone();
}
