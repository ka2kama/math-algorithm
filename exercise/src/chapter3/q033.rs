#![allow(nonstandard_style)]

use num_traits::ToPrimitive;
use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Component {
    x: i64,
    y: i64,
}

fn main() {
    input! { A: Point, B: Point, C: Point };

    let ans = solve(A, B, C);
    println!("{:.7}", ans);
}

fn solve(A: Point, B: Point, C: Point) -> f64 {
    let BA = Component {
        x: A.x - B.x,
        y: A.y - B.y,
    };
    let BC = Component {
        x: C.x - B.x,
        y: C.y - B.y,
    };

    let ip_BA_BC = BA.x * BC.x + BA.y * BC.y;
    if ip_BA_BC <= 0 {
        return (BA.x.pow(2) + BA.y.pow(2)).to_f64().unwrap().sqrt();
    }

    let CA = Component {
        x: A.x - C.x,
        y: A.y - C.y,
    };
    let CB = Component {
        x: B.x - C.x,
        y: B.y - C.y,
    };

    let ip_CA_CB = CA.x * CB.x + CA.y * CB.y;
    if ip_CA_CB <= 0 {
        return (CA.x.pow(2) + CA.y.pow(2)).to_f64().unwrap().sqrt();
    }

    let op_BA_BC = (BA.x * BC.y - BA.y * BC.x).abs();
    let dist_BC = (BC.x.pow(2) + BC.y.pow(2)).to_f64().unwrap().sqrt();
    op_BA_BC.to_f64().unwrap() / dist_BC
}
