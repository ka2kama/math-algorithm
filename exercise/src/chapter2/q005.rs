#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: usize, A: [i64; N], };
    let s: i64 = A.into_iter().sum();
    println!("{}", s % 100);
}
