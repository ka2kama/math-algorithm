#![allow(nonstandard_style)]

use proconio::input;

const F: f64 = 10_000_000.;

fn main() {
    input! { N: usize, B: [f64; N], R: [f64; N] };
    let n = N as f64;
    let (blue, red) = B
        .into_iter()
        .zip(R)
        .fold((0., 0.), |(exp_b, exp_r), (b, r)| {
            (exp_b + b * F / n, exp_r + r * F / n)
        });

    let ans = (blue + red) / F;
    println!("{:.7}", ans);
}
