#![allow(nonstandard_style)]

use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug)]
struct Q {
    p: f64,
    q: f64,
}

const F: f64 = 10_000_000.;

fn main() {
    input! { N: usize, A: [Q; N], };
    let s = A.into_iter().fold(0., |s, Q { p, q }| s + q * F / p);

    println!("{:.7}", s / F);
}
