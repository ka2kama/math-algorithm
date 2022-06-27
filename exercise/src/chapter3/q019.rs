#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, A: [u64; N] };
    let (mut red, mut yellow, mut blue) = (0_usize, 0_usize, 0_usize);
    for x in A {
        match x {
            1 => red += 1,
            2 => yellow += 1,
            3 => blue += 1,
            _ => {}
        }
    }

    let ans = count(red) + count(yellow) + count(blue);
    println!("{}", ans);
}

fn count(n: usize) -> usize {
    n * (n - 1) / 2
}
