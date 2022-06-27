#![allow(nonstandard_style)]

use itertools::Itertools;
use num_traits::ToPrimitive;
use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    input! { N: usize, A: [Point; N] };

    let ans = A
        .into_iter()
        .combinations(2)
        .map(|v| {
            ((v[1].x - v[0].x).pow(2) + (v[1].y - v[0].y).pow(2))
                .to_f64()
                .unwrap()
                .sqrt()
        })
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap();
    println!("{:.10}", ans);
}
