mod trees;

fn main() {
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