/*
MIT License

Copyright (c) 2019 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use rand::Rng;

mod binheap;
mod trees;

fn main() {
	main1();
	main2();
	main3();
}

fn main1() {
	println!("Binary Max Heap:");

	let mut t = [10, 8, 1, 5, 12, 14, 20, 2, 9, 13];

	binheap::build_max_heap(&mut t);
	binheap::print(&t);
}

fn main2() {
	println!("Binary Search Tree:");

	let mut t = trees::Bst::new(10);
	t.insert(8);
	t.insert(1);
	t.insert(5);
	t.insert(12);
	t.insert(14);
	t.insert(20);
	t.insert(2);
	t.insert(9);
	t.insert(13);

	t.print();
}

fn main3() {
	println!("Binary Search Tree:");

	let max_ = std::u64::MAX;
	let media = max_ / 2;

	let mut t = trees::Bst::new(media);
	let mut rng = rand::thread_rng();
	let mut x;

	for _i in 0..9999 {
		x = rng.gen_range(0, max_);
		t.insert(x);
	}

	t.print();
}
