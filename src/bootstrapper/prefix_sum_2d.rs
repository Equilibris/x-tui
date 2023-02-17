use tui::layout::Rect;

#[derive(Clone)]
pub struct PrefixSum2d {
    sz: Rect,
    ir: Vec<isize>,

    fast_clear_locs: Vec<usize>,

    inner_bound: Option<(u16, u16, u16, u16)>,
}

pub struct PrefixSum2dIterator<'a> {
    parent: &'a PrefixSum2d,

    current_row: isize,
    columns: Vec<isize>,

    idx: usize,
}

impl<'a> Iterator for PrefixSum2dIterator<'a> {
    type Item = (u16, u16, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.parent.ir.len() {
            return None;
        }
        if let Some((min_x, min_y, max_x, max_y)) = self.parent.inner_bound {
            let w = self.parent.sz.width;

            let x = self.idx as u16 % w;
            let y = self.idx as u16 / w;

            // x will always equal zero here then
            // if x < min_x {
            //     self.idx += (min_x - x) as usize;

            //     return self.next();
            // }
            // if max_x <= x {
            //     // self.idx += (min_x - x) as usize;
            //     self.idx += (w - x + min_x) as usize;

            //     return self.next();
            // }
            // // This is handled at startup
            // if y < min_y {
            //     self.idx = (min_y * w + min_x) as usize;

            //     return self.next();
            // }
            // if max_y <= y {
            //     return None;
            // }

            let v = self.parent.ir[self.idx];

            let xu = x as usize;

            if x == 0 {
                self.current_row = 0;
            }

            self.current_row += v;

            self.columns[xu] += self.current_row;

            self.idx += 1;

            Some((x, y, self.columns[xu]))
        } else {
            None
        }
    }
}

impl PrefixSum2d {
    pub fn new(sz: Rect) -> Self {
        Self {
            sz,
            ir: vec![0; sz.height as usize * sz.width as usize],
            fast_clear_locs: vec![],

            inner_bound: None,
        }
    }

    pub fn insert(&mut self, bound: Rect) {
        match self.inner_bound {
            Some(mut v) => {
                v.0 = std::cmp::min(v.0, bound.x);
                v.1 = std::cmp::min(v.1, bound.y);

                // Can maybe sub one
                v.2 = std::cmp::max(v.2, bound.x + bound.width);
                v.3 = std::cmp::max(v.3, bound.y + bound.height);
            }
            None => {
                self.inner_bound = Some((
                    bound.x,
                    bound.y,
                    bound.x + bound.width - 1,
                    bound.y + bound.height - 1,
                ));
            }
        }

        if self.sz.height < bound.y || self.sz.width < bound.x {
            return;
        }

        let i = bound.y * self.sz.width + bound.x;
        let i = i as usize;

        if self.ir[i] == 0 {
            self.fast_clear_locs.push(i)
        }

        self.ir[i] += 1;

        let mut ca = false;
        if bound.y + bound.height < self.sz.height {
            ca = true;
            let i = (bound.y + bound.height) * self.sz.width + bound.x;
            let i = i as usize;

            if self.ir[i] == 0 {
                self.fast_clear_locs.push(i)
            }

            self.ir[i] -= 1;
        }

        if bound.x + bound.width < self.sz.width {
            ca &= true;
            let i = bound.y * self.sz.width + bound.x + bound.width;
            let i = i as usize;

            if self.ir[i] == 0 {
                self.fast_clear_locs.push(i)
            }

            self.ir[i] -= 1;
        }
        if ca {
            let i = (bound.y + bound.height) * self.sz.width + bound.x + bound.width;
            let i = i as usize;

            if self.ir[i] == 0 {
                self.fast_clear_locs.push(i)
            }

            self.ir[i] += 1;
        }
    }

    pub fn iter(&self) -> PrefixSum2dIterator<'_> {
        // match self.inner_bound {
        //     Some((min_x, min_y, max_x, max_y)) => PrefixSum2dIterator {
        //         parent: self,

        //         current_row: 0,
        //         columns: vec![0; self.sz.width as usize],
        //         idx: (min_y * self.sz.width + min_x) as usize,
        //     },
        //     None => PrefixSum2dIterator {
        //         parent: self,

        //         current_row: 0,
        //         columns: vec![0; self.sz.width as usize],
        //         idx: 0,
        //     },
        // }

        PrefixSum2dIterator {
            parent: self,

            current_row: 0,
            columns: vec![0; self.sz.width as usize],
            idx: 0,
        }
    }

    pub fn clear(&mut self) {
        for loc in std::mem::take(&mut self.fast_clear_locs) {
            self.ir[loc] = 0;
        }

        self.inner_bound = None;
    }

    pub fn resize(&mut self, sz: Rect) {
        self.clear();

        self.ir.resize(sz.height as usize * sz.width as usize, 0);
    }
}

#[cfg(test)]
mod tests {
    use tui::layout::Rect;

    use super::PrefixSum2d;

    #[test]
    fn one_x_one() {
        let sz = Rect::new(0, 0, 1, 1);

        let mut base = PrefixSum2d::new(sz);
        base.insert(sz);

        assert_eq!(base.ir, vec![1]);
    }

    #[test]
    fn two_x_two() {
        let sz = Rect::new(0, 0, 2, 2);

        let mut base = PrefixSum2d::new(sz);
        base.insert(Rect::new(0, 0, 1, 1));

        let mut iterator = base.iter();

        assert_eq!(iterator.next(), Some((0, 0, 1)));

        for (index, i) in iterator.enumerate() {
            let index = index + 1;
            assert_eq!(i, (index as u16 % 2, index as u16 / 2, 0));
        }

        base.insert(sz);

        let mut iterator = base.iter();

        assert_eq!(iterator.next(), Some((0, 0, 2)));

        for (index, i) in iterator.enumerate() {
            let index = index + 1;
            assert_eq!(i, (index as u16 % 2, index as u16 / 2, 1));
        }

        base.insert(Rect::new(1, 1, 1, 1));

        let mut iterator = base.iter();

        assert_eq!(iterator.next(), Some((0, 0, 2)));
        assert_eq!(iterator.next(), Some((1, 0, 1)));
        assert_eq!(iterator.next(), Some((0, 1, 1)));
        assert_eq!(iterator.next(), Some((1, 1, 2)));

        base.clear();

        let mut iterator = base.iter();

        assert_eq!(iterator.next(), None);
        // // Caught by the no-value condition
        // assert_eq!(iterator.next(), Some((0, 0, 0)));
        // assert_eq!(iterator.next(), Some((1, 0, 0)));
        // assert_eq!(iterator.next(), Some((0, 1, 0)));
        // assert_eq!(iterator.next(), Some((1, 1, 0)));
    }

    #[test]
    fn three_x_three() {
        let sz = Rect::new(0, 0, 3, 3);

        let mut base = PrefixSum2d::new(sz);
        base.insert(Rect::new(0, 0, 1, 1));

        let mut iterator = base.iter().map(|(_, _, v)| v).collect::<Vec<_>>();

        assert_eq!(iterator, vec![1, 0, 0, 0, 0, 0, 0, 0, 0])
    }
}
