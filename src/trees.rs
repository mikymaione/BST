pub struct Bst {
	pub k: i64,

	pub sx: Option<Box<Bst>>,
	pub dx: Option<Box<Bst>>,
}

impl Bst {
	fn new_leaf(v: i64) -> Option<Box<Bst>> {
		Option::Some(Box::new(Bst::new(v)))
	}

	pub fn new(v: i64) -> Self {
		Bst {
			k: v,
			sx: None,
			dx: None,
		}
	}

	pub fn insert(&mut self, v: i64) {
		if self.k > v {
			match self.dx {
				Some(ref mut p) => p.insert(v),
				None => self.dx = Bst::new_leaf(v),
			}
		} else {
			match self.sx {
				Some(ref mut p) => p.insert(v),
				None => self.sx = Bst::new_leaf(v),
			}
		}
	}

	fn print_it(&self, l: u8) {
		for _i in 0..l {
			print!("-");
		}

		println!("{}", self.k);

		match self.sx {
			Some(ref p) => p.print_it(l + 1),
			_ => (),
		}

		match self.dx {
			Some(ref p) => p.print_it(l + 1),
			_ => (),
		}
	}

	pub fn print(&self) {
		self.print_it(0);
	}
}
