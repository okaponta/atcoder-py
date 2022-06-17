use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n:usize,
        q:usize,
        a:[usize;n],
        b:[usize;n],
        hw:[(Usize1,Usize1,Usize1,Usize1);q],
    }
    let mut hseg = SegmentTree::new(n - 1, 0, |a, b| gcd(a, b));
    let mut wseg = SegmentTree::new(n - 1, 0, |a, b| gcd(a, b));
    for i in 0..n - 1 {
        hseg.update(i, abs_sub(a[i], a[i + 1]));
        wseg.update(i, abs_sub(b[i], b[i + 1]));
    }
    for (h1, h2, w1, w2) in hw {
        let a = a[h1] + b[w1];
        let b = hseg.query(h1..h2);
        let c = wseg.query(w1..w2);
        println!("{}", gcd(a, gcd(b, c)));
    }
}

pub struct SegmentTree<T, F> {
    seg: Vec<T>,
    n: usize,
    f: F,
    initial_value: T,
}

impl<T: Copy, F> SegmentTree<T, F>
where
    F: Fn(T, T) -> T,
{
    pub fn new(size: usize, initial_value: T, f: F) -> SegmentTree<T, F> {
        let mut m = 1;
        while m <= size {
            m <<= 1;
        }
        SegmentTree {
            seg: vec![initial_value; m * 2],
            n: m,
            f,
            initial_value,
        }
    }

    pub fn update(&mut self, k: usize, value: T) {
        let mut k = k;
        k += self.n - 1;
        self.seg[k] = value;
        while k > 0 {
            k = (k - 1) >> 1;
            self.seg[k] = (self.f)(self.seg[k * 2 + 1], self.seg[k * 2 + 2]);
        }
    }

    // 半開区完なので注意
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        self.query_range(range, 0, 0..self.n)
    }

    fn query_range(
        &self,
        range: std::ops::Range<usize>,
        k: usize,
        seg_range: std::ops::Range<usize>,
    ) -> T {
        if seg_range.end <= range.start || range.end <= seg_range.start {
            self.initial_value
        } else if range.start <= seg_range.start && seg_range.end <= range.end {
            self.seg[k]
        } else {
            let mid = (seg_range.start + seg_range.end) >> 1;
            let x = self.query_range(range.clone(), k * 2 + 1, seg_range.start..mid);
            let y = self.query_range(range, k * 2 + 2, mid..seg_range.end);
            (self.f)(x, y)
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn abs_sub(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
