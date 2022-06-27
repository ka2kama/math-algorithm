#![allow(nonstandard_style)]

use proconio::input;

const F: f64 = 10_000_000.;

fn main() {
    input! { N: usize, };

    let s: f64 = (1..=N).rev().map(|i| (N as f64) * F / (i as f64)).sum();
    println!("{:.7}", s / F);
}
