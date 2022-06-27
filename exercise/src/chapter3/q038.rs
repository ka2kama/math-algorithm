#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, Q: usize, A: [i64; N], LR: [(usize, usize); Q] };

    let mut v = vec![0_i64; N + 1];
    for (i, x) in A.into_iter().enumerate() {
        v[i + 1] = v[i] + x;
    }

    for (l, r) in LR {
        println!("{}", v[r] - v[l - 1]);
    }
}
