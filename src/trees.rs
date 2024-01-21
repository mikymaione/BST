/*
MIT License

Copyright (c) 2019 Michele Maione

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use std::fmt;

pub struct Bst {
    k: u64,
    left: Option<Box<Bst>>,
    right: Option<Box<Bst>>,
}

impl fmt::Display for Bst {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.print(fmt, 0)
    }
}

impl Bst {
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

impl Bst {
    fn new_leaf(v: u64) -> Option<Box<Bst>> {
        Some(Box::new(Bst::new(v)))
    }

    pub fn new(v: u64) -> Self {
        Bst {
            k: v,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, v: u64) {
        if self.k > v {
            match self.right {
                None =>
                    self.right = Bst::new_leaf(v),

                Some(ref mut p) =>
                    p.insert(v),
            }
        } else {
            match self.left {
                None =>
                    self.left = Bst::new_leaf(v),

                Some(ref mut p) =>
                    p.insert(v),
            }
        }
    }
}
