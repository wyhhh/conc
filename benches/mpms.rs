#![feature(test)]
extern crate test;
use conc::channel::mpmc::DefaultChannel;
use conc::channel::MpmcChannel;
use conc::channel::Sender;
use conc::channel::Receiver;


#[bench]
fn bench(b: &mut test::Bencher) {
	// exact code to benchmark must be passed as a closure to the iter
	// method of Bencher
	b.iter(|| {
		let (tx, rx) = DefaultChannel::unbounded();

		crossbeam_utils::thread::scope(|s| {
			for n in 0..10 {
				let tx = tx.clone();

				s.spawn(move |_| {
					tx.send(n);
				});

				for n in 0..10 {
					let rx = rx.clone();

					s.spawn(move |_| {
						let res = rx.recv();
						println!("{:?}", res);
					});
				}
			}
		})
		.unwrap();
	})
}

// #[bench]
// fn bench_crossbeam(b: &mut test::Bencher) {
// 	// exact code to benchmark must be passed as a closure to the iter
// 	// method of Bencher
// 	b.iter(|| {
// 		let (tx, rx) = c::unbounded();

// 		crossbeam_utils::thread::scope(|s| {
// 			for n in 0..10 {
// 				let tx = tx.clone();

// 				s.spawn(move |_| {
// 					tx.send(n);
// 				});

// 				for n in 0..10 {
// 					let rx = rx.clone();

// 					s.spawn(move |_| {
// 						let res = rx.recv();
// 						println!("{:?}", res);
// 					});
// 				}
// 			}
// 		})
// 		.unwrap();
// 	})
// }