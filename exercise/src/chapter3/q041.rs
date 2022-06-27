#![allow(nonstandard_style)]

use std::fmt::Display;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { T: usize, N: usize, A: [(usize, usize); N], };

    let ans = solve(A, T);
    for n in ans {
        println!("{}", n);
    }
}

fn solve(A: Vec<(usize, usize)>, T: usize) -> Vec<i64> {
    let mut diff = vec![0; T + 2];
    for (l, r) in A {
        diff[l + 1] += 1;
        diff[r + 1] -= 1;
    }

    let mut nums = vec![0; T + 1];
    for (i, d) in diff.drain(1..=T).enumerate() {
        nums[i + 1] = nums[i] + d;
    }
    nums.drain(..1);
    nums
}
