#![allow(nonstandard_style)]

use proconio::input;

const F: f64 = 10_000.;
const COEFF_A: f64 = 2. / 6.;
const COEFF_B: f64 = 4. / 6.;

fn main() {
    input! { N: usize, A: [f64; N], B: [f64; N] };
    let s = A
        .into_iter()
        .zip(B)
        .fold(0., |s, (a, b)| s + a * F * COEFF_A + b * F * COEFF_B);

    println!("{:.4}", s / F);
}
