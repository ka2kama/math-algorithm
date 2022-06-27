#![allow(nonstandard_style)]

use im_rc::hashset;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: u64, };
    let iter = (1_u64..)
        .take_while(|&i| i.pow(2) <= N)
        .filter(|&i| N % i == 0)
        .flat_map(|i| hashset![i, N / i]);

    for i in iter {
        println!("{}", i);
    }
}
