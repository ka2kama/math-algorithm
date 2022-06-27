#![allow(nonstandard_style)]

use itertools::Itertools;
use proconio::input;

fn main() {
    input! { n: u64, r: usize };
    let ans = (1..=n).combinations(r).count();
    println!("{}", ans);
}
