fn left(i: usize) -> usize {
	return 2 * i;
}

fn right(i: usize) -> usize {
	return (2 * i) + 1;
}

fn max_heapify(a: &mut [u8], heap_size: usize, i: usize) {
	let mut massimo;
	let l = left(i);
	let r = right(i);

	if l <= heap_size && a[l - 1] > a[i - 1] {
		massimo = l
	} else {
		massimo = i;
	}

	if r <= heap_size && a[r - 1] > a[massimo - 1] {
		massimo = r;
	}

	if massimo != i {
		let tmp = a[i - 1];
		a[i - 1] = a[massimo - 1];
		a[massimo - 1] = tmp;

		max_heapify(a, heap_size, massimo);
	}
}

pub fn build_max_heap(a: &mut [u8]) {
	let heap_size = a.len();
	let size = a.len() / 2;

	for i in (1..size).rev() {
		max_heapify(a, heap_size, i);
	}
}

fn print_it(a: &[u8], i: usize, l: u8) {
	if i + 1 > a.len() {
		return;
	}

	for _i in 0..l {
		print!("-");
	}

	println!("{}", a[i - 1]);

	let sx = left(i);
	let dx = right(i);

	print_it(a, sx, l + 1);
	print_it(a, dx, l + 1);
}

pub fn print(a: &[u8]) {
	print_it(a, 1, 0);
}
