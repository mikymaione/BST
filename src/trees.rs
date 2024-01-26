/*
MIT License

Copyright (c) 2019 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use std::fmt;
use std::fmt::Display;

type MemoryContainer<K> = Box<Bst<K>>;
type Node<K> = Option<MemoryContainer<K>>;

pub struct Tree<K> {
    root: Node<K>,
}

pub struct Bst<K> {
    k: K,
    left: Node<K>,
    right: Node<K>,
}

impl<K: Ord + Display> Display for Tree<K> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.print(fmt, 0)
    }
}

impl<K: Ord + Display> Display for Bst<K> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.print(fmt, 0)
    }
}

impl<K: Ord + Display> Tree<K> {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn insert(&mut self, k: K) {
        match self.root {
            None =>
                self.root = Bst::new_leaf(k),

            Some(ref mut root) =>
                root.insert(k),
        }
    }

    pub fn delete(&mut self, k: K) {
        if let Some(root) = self.root.take() {
            self.root = Bst::delete(root, k);
        }
    }

    fn print(&self, fmt: &mut fmt::Formatter, l: u64) -> fmt::Result {
        if let Some(ref p) = self.root {
            p.print(fmt, l)?
        }

        Ok(())
    }
}

impl<K: Ord + Display> Bst<K> {
    pub fn new(k: K) -> Self {
        Self {
            k,
            left: None,
            right: None,
        }
    }

    fn new_leaf(k: K) -> Node<K> {
        Some(Box::new(Self::new(k)))
    }

    pub fn insert(&mut self, k: K) {
        if k < self.k {
            match self.left {
                None =>
                    self.left = Self::new_leaf(k),

                Some(ref mut p) =>
                    p.insert(k),
            }
        } else {
            match self.right {
                None =>
                    self.right = Self::new_leaf(k),

                Some(ref mut p) =>
                    p.insert(k),
            }
        }
    }

    fn delete(mut t: MemoryContainer<K>, k: K) -> Node<K> {
        if k == t.k {
            match (t.left.take(), t.right.take()) {
                (None, None) =>
                    None,

                (Some(left), None) =>
                    Some(left),

                (None, Some(right)) =>
                    Some(right),

                (Some(mut left), Some(right)) => {
                    match left.rightmost_child() {
                        Some(mut rightmost) => {
                            rightmost.left = Some(left);
                            rightmost.right = Some(right);
                            Some(rightmost)
                        }
                        None => {
                            left.right = Some(right);
                            Some(left)
                        }
                    }
                }
            }
        } else {
            if k < t.k {
                if let Some(left) = t.left.take() {
                    t.left = Self::delete(left, k);
                }
            } else {
                if let Some(right) = t.right.take() {
                    t.right = Self::delete(right, k);
                }
            }

            Some(t)
        }
    }

    fn rightmost_child(&mut self) -> Node<K> {
        match self.right {
            None =>
                None,

            Some(ref mut right) =>
                match right.rightmost_child() {
                    Some(rightmost) =>
                        Some(rightmost),

                    None => {
                        let mut right = self.right.take();

                        if let Some(ref mut right) = right {
                            // right <- right.left
                            // right.left <- None
                            self.right = right.left.take();
                        }

                        right
                    }
                }
        }
    }

    fn print(&self, fmt: &mut fmt::Formatter, l: u64) -> fmt::Result {
        for _i in 0..l {
            fmt.write_str("-")?;
        }

        fmt.write_fmt(format_args!("{}\n", self.k))?;

        if let Some(ref p) = self.left {
            p.print(fmt, l + 1)?
        };

        if let Some(ref p) = self.right {
            p.print(fmt, l + 1)?
        };

        Ok(())
    }
}
