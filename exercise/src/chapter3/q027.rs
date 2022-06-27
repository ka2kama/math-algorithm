#![allow(nonstandard_style)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! { N: usize, mut A: [i64; N] };
    MergeSort::run(&mut A);
    println!("{}", A.iter().join(" "));
}

struct MergeSort<'a, T>
where
    T: PartialOrd + Copy,
{
    v: &'a mut [T],
}

impl<'a, T> MergeSort<'a, T>
where
    T: PartialOrd + Copy,
{
    fn run(v: &'a mut [T]) {
        let len = v.len();
        let mut ms = Self { v };
        ms.sort(0, len);
    }

    fn sort(&mut self, l: usize, r: usize) {
        if r - l == 1 {
            return;
        }

        let m = (l + r) / 2;
        self.sort(l, m);
        self.sort(m, r);

        let c = self.merge(l, r, m);

        for (i, x) in c.into_iter().enumerate() {
            self.v[l + i] = x;
        }
    }

    fn merge(&self, l: usize, r: usize, m: usize) -> Vec<T> {
        let mut c = Vec::new();
        let (mut c1, mut c2) = (l, m);
        while c1 != m || c2 != r {
            if c1 == m {
                c.push(self.v[c2]);
                c2 += 1;
            } else if c2 == r {
                c.push(self.v[c1]);
                c1 += 1;
            } else {
                if self.v[c1] < self.v[c2] {
                    c.push(self.v[c1]);
                    c1 += 1;
                } else {
                    c.push(self.v[c2]);
                    c2 += 1;
                }
            }
        }
        c
    }
}
