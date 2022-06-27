#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: i64, };
    let p: i64 = (1..=N).product();
    println!("{}", p);
}
