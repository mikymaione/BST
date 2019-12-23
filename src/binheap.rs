fn left(i: usize) -> usize {
	return (2 * i) + 1;
}

fn right(i: usize) -> usize {
	return (2 * i) + 2;
}

fn print_it(a: &[u8], i: usize, l: u8) {
	if i + 1 > a.len() {
		return;
	}
	for _i in 0..l {
		print!("-");
	}

	println!("{}", a[i]);

	let sx = left(i);
	let dx = right(i);

	print_it(a, sx, l + 1);
	print_it(a, dx, l + 1);
}

pub fn print(a: &[u8]) {
	print_it(a, 0, 0);
}
