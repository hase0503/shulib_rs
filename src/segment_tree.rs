trait Monoid {
    type S: Clone;
    fn id() -> Self::S;
    fn bi_op(a: &Self::S, b: &Self::S) -> Self::S;
}

struct SegmentTree<M: Monoid> {
    size: usize,
    tree: Vec<M::S>,
}

impl<M: Monoid> SegmentTree<M> {
    fn new(n: usize) -> Self {
        let size = n.next_power_of_two();
        Self {
            size,
            tree: vec![M::id(); 2 * size],
        }
    }

    fn from(v: Vec<M::S>) -> Self {
        let n = v.len();
        let size = n.next_power_of_two();
        let mut ret = SegmentTree::new(n);
        for i in 0..size {
            ret.set(i, v[i]);
        }
        ret
    }

    fn get(&mut self, mut pos: usize) -> M::S {
        pos += self.size;
        self.tree[pos]
    }

    fn set(&mut self, mut pos: usize, x: M::S) {
        pos += self.size;
        self.tree[pos] = x;
        while pos / 2 > 0 {
            pos /= 2;
            self._update(pos);
        }
    }

    fn apply(&mut self, pos: usize, x: M::S) {
        self.set(pos, M::bi_op(&self.tree[self.size + pos], &x));
    }

    fn _update(&mut self, pos: usize) {
        self.tree[pos] = M::bi_op(&self.tree[2 * pos], &self.tree[2 * pos + 1]);
    }

    fn prod(&self, mut left: usize, mut right: usize) -> M::S {
        left += self.size;
        right += self.size;

        let (l, r) = (M::id(), M::id());
        while left < right {
            if left % 2 == 1 {
                l = M::bi_op(&l, &self.tree[left]);
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                r = M::bi_op(&r, &self.tree[right]);
            }
            left /= 2;
            right /= 2;
        }

        M::bi_op(&l, &r)
    }

    fn all_prod(&self) -> M::S {
        if self.size == 0 {
            M::id()
        } else {
            self.tree[1]
        }
    }
}
