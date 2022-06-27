#![allow(nonstandard_style)]

use num_traits::ToPrimitive;
use proconio::derive_readable;
use proconio::input;
use rand::distributions::weighted::alias_method::Weight;

#[derive_readable]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let d = (other.x - self.x).pow(2) + (other.y - self.y).pow(2);
        d.to_f64().unwrap().sqrt()
    }
}

fn main() {
    input! { N: usize, A: [Point; N] };

    let mut min_dist = f64::MAX;
    for (i, p1) in A.iter().enumerate() {
        for p2 in &A[i + 1..] {
            let d = p1.distance(p2);
            min_dist = min_dist.min(d);
        }
    }

    println!("{:.10}", min_dist);
}
