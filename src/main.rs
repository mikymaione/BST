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
