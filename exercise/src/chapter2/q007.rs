#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: i64, X: i64, Y: i64, };
    let ans = (1..=N).filter(|&m| m % X == 0 || m % Y == 0).count();
    println!("{}", ans);
}
