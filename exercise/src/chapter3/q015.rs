#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { A: u64, B: u64 };
    println!("{}", gcd(A, B));
}

fn gcd(A: u64, B: u64) -> u64 {
    let mut a = A;
    let mut b = B;
    while a >= 1 && b >= 1 {
        if a < b {
            b = b % a;
        } else {
            a = a % b;
        }
    }
    if a >= 1 {
        a
    } else {
        b
    }
}
