use threadpool::ThreadPool;
use std::sync::mpsc::channel;

// Used for verifying that the order is saved. Not needed in final result
// Left in to show the development process
// See Below
//use std::{thread, time};

fn main() {
	// Start with small vector below threshold
	let vector = vec![1, 2, 3];
	process(vector);
	
	// Then process with one above threshold
	let vector2 = vec![1, 2, 3, 4, 5];
	process(vector2);
}

fn process(vector: Vec<i32>) {
	let threshold = 5;

	if vector.len() < threshold {
		println!("Threshold Not Met, Processing Without Parallelism");
		process_non_parallel(vector);
	} else {
		println!("Threshold Met, Processing With Thread Pool");
		process_parallel(vector);
	}
}

fn process_non_parallel(vector: Vec<i32>) {
	let mut result_vec = vec![0; vector.len()];
	for i in 0..vector.len() {
		result_vec[i] = process_vec(vector[i]);
	}

	for (pos, e) in result_vec.iter().enumerate() {
		println!("Element at position {}: {:?}", pos, e);
	}
}

fn process_parallel(vector: Vec<i32>) {
	let n_workers = 4;
	let pool = ThreadPool::new(n_workers);

	let mut result_vec_parallel = vec![0; vector.len()];
	let (tx, rx) = channel();
	for i in 0..vector.len() {
		let tx = tx.clone();
		let e = vector[i];

		pool.execute(move|| {
			let result_element = process_vec(e);

			// Used for verifying that the order is saved. Not needed in final result.
			// Left in to show the development process
			//let millis = time::Duration::from_millis(e as u64 * 1000);
			//let now = time::Instant::now();
			//thread::sleep(millis);

			tx.send(vec![result_element, i as i32]).unwrap();
		});
	}

	for _ in 0..vector.len() {
		let r = rx.recv().unwrap();
        result_vec_parallel[r[1] as usize] = r[0];
    }

    for i in 0..result_vec_parallel.len() {
		println!("Element at position {}: {}", i, result_vec_parallel[i]);
	}
}

fn process_vec(element: i32) -> i32 {
	return element;
}
