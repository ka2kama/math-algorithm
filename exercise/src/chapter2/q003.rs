#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: usize, A: [i64; N], };
    println!("{}", A.into_iter().sum::<i64>());
}
