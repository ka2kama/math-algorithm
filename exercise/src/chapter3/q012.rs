#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: u64, };
    let ans = if is_prime(N) { "Yes" } else { "No" };
    println!("{}", ans);
}

fn is_prime(n: u64) -> bool {
    (2_u64..).take_while(|&i| i.pow(2) <= n).all(|i| n % i != 0)
}
