#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: usize, X: usize, A: [usize; N] };

    println!("{}", if A.contains(&X) { "Yes" } else { "No" });
}
