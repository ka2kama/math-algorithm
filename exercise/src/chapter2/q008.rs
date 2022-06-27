#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: i64, S: i64, };
    let ans = (1..=N)
        .flat_map(|a| (1..=N).map(move |b| a + b))
        .filter(|&x| x <= S)
        .count();
    println!("{}", ans);
}
