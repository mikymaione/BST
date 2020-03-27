/*
MIT License

Copyright (c) 2019 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
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
