#![allow(nonstandard_style)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! { N: i64, };
    let ans = (2..=N).filter(|&x| (2..x).all(|y| x % y != 0)).join(" ");
    println!("{}", ans);
}
