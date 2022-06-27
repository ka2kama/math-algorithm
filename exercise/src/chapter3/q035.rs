#![allow(nonstandard_style)]

use num_traits::ToPrimitive;
use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug, Eq, PartialEq)]
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

#[derive_readable]
#[derive(Debug, Eq, PartialEq)]
struct Circle {
    center: Point,
    r: i64,
}

fn main() {
    input! { A: Circle, B: Circle };
    let ans = solve(A, B);
    println!("{}", ans);
}

fn solve(A: Circle, B: Circle) -> u8 {
    if A == B {
        return 2;
    }

    let center_dist = A.center.distance(&B.center);
    let r_sum = (A.r + B.r).to_f64().unwrap();
    if center_dist > r_sum {
        5
    } else if center_dist == r_sum {
        4
    } else {
        let (c1, c2) = if A.r < B.r { (A, B) } else { (B, A) };
        let (r1, r2) = (c1.r.to_f64().unwrap(), c2.r.to_f64().unwrap());
        if center_dist + r1 < r2 {
            1
        } else if center_dist + r1 > r2 {
            3
        } else {
            2
        }
    }
}
