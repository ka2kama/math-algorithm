#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, A: [u64; N] };
    let (mut cnt_100, mut cnt_200, mut cnt_300, mut cnt_400) = (0_usize, 0_usize, 0_usize, 0_usize);

    for x in A {
        match x {
            100 => cnt_100 += 1,
            200 => cnt_200 += 1,
            300 => cnt_300 += 1,
            400 => cnt_400 += 1,
            _ => {}
        }
    }

    println!("{}", cnt_100 * cnt_400 + cnt_200 * cnt_300);
}
