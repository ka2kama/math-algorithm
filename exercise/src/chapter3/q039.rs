#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, Q: usize,  A: [(usize, usize, i64); Q] };

    let mut list_of_diff_from_next = vec![0_i64; N + 2];
    for (l, r, x) in A {
        list_of_diff_from_next[l] += x;
        list_of_diff_from_next[r + 1] -= x;
    }

    for &diff_from_next in &list_of_diff_from_next[2..=N] {
        let s = if diff_from_next > 0 {
            "<"
        } else if diff_from_next < 0 {
            ">"
        } else {
            "="
        };
        print!("{}", s);
    }

    println!()
}
