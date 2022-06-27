#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, A: [u64; N] };
    let ans = A[2..].iter().fold(lcm(A[0], A[1]), |a, &b| lcm(a, b));
    println!("{}", ans);
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
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
