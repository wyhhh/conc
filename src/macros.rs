macro_rules! shared {
    ($($name: ident),+) => {$(
		#[derive(Default, Debug)]
		pub struct $name(std::sync::Arc<$crate::$name>);

		impl $name {
			pub fn new() -> Self {
				Default::default()
			}
		}

		impl Clone for $name {
			fn clone(&self) -> Self {
				$name(self.0.clone())
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

macro_rules! shared_generic {
    ($($name: ident),+) => {$(
		#[derive(Default, Debug)]
		pub struct $name<T>(std::sync::Arc<$crate::$name<T>>);

		impl<T: Default> $name<T> {
			pub fn new() -> Self {
				Default::default()
			}
		}

		impl<T> Clone for $name<T> {
			fn clone(&self) -> Self {
				$name(self.0.clone())
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

pub mod shared {
	shared!(Sema);
	shared_generic!(BoundedQ, UnboundedQ);
}


#[test]
fn test() {
	let q = shared::BoundedQ::new();

	q.push(2);

	let q2 = q.clone();

	println!("{:?}",q2);

	let s = shared::Sema::new();
	s.release();

	println!("{:?}",s);
	let _s2 = s.clone();
}
