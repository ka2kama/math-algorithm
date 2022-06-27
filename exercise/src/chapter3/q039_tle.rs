#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, Q: usize,  A: [(usize, usize, i64); Q] };

    let mut v = vec![0_i64; N];
    for (l, r, x) in A {
        for i in l - 1..=r - 1 {
            v[i] += x;
        }
    }

    for (&prev, &next) in v.iter().zip(v.iter().skip(1)) {
        let ans = if prev < next {
            "<"
        } else if prev > next {
            ">"
        } else {
            "="
        };
        print!("{}", ans);
    }

    println!()
}
