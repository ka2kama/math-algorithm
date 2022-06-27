#![allow(nonstandard_style)]

use proconio::{derive_readable, fastout, input};

#[derive_readable]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Point2d {
    x: i64,
    y: i64,
}

impl Point2d {
    fn to(&self, other: &Point2d) -> Vector {
        Vector {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn cross(&self, other: &Self) -> i64 {
        self.x * other.y - self.y * other.x
    }
}

#[fastout]
fn main() {
    input! { A: Point2d, B: Point2d, C: Point2d, D: Point2d };

    let ans = if solve(&A, &B, &C, &D) { "Yes" } else { "No" };
    println!("{}", ans);
}

fn solve(A: &Point2d, B: &Point2d, C: &Point2d, D: &Point2d) -> bool {
    let AB = A.to(&B);
    let AC = A.to(&C);
    let AD = A.to(&D);
    let CA = C.to(&A);
    let CB = C.to(&B);
    let CD = C.to(&D);

    let cross_ABC = AB.cross(&AC);
    let cross_ABD = AB.cross(&AD);
    let cross_CDA = CD.cross(&CA);
    let cross_CDB = CD.cross(&CB);

    let clockwise_ABC = cross_ABC > 0;
    let counterclockwise_ABC = cross_ABC < 0;
    let same_straight_line_ABC = cross_ABC == 0;

    let clockwise_ABD = cross_ABD > 0;
    let counterclockwise_ABD = cross_ABD < 0;
    let same_straight_line_ABD = cross_ABD == 0;

    let clockwise_CDA = cross_CDA > 0;
    let counterclockwise_CDA = cross_CDA < 0;
    let same_straight_line_CDA = cross_CDA == 0;

    let clockwise_CDB = cross_CDB > 0;
    let counterclockwise_CDB = cross_CDB < 0;
    let same_straight_line_CDB = cross_CDB == 0;

    // すべての点が同一直線上にある場合
    if same_straight_line_ABC
        && same_straight_line_ABD
        && same_straight_line_CDA
        && same_straight_line_CDB
    {
        let (a, b) = if A <= B { (A, B) } else { (B, A) };
        let (c, d) = if C <= D { (C, D) } else { (D, C) };
        a.max(c) <= b.min(d)
    } else {
        let clockwise_or_same_line_ABC = clockwise_ABC || same_straight_line_ABC;
        let counterclockwise_or_same_line_ABC = counterclockwise_ABC || same_straight_line_ABC;
        let clockwise_or_same_line_ABD = clockwise_ABD || same_straight_line_ABD;
        let counterclockwise_or_same_line_ABD = counterclockwise_ABD || same_straight_line_ABD;

        // 線分ABを充分に伸ばした場合、線分CDと交差するか。
        // 一方がABに対して時計回りかつ、もう一方が反時計回りの場合は交差する
        let does_AB_sep_CD = (clockwise_or_same_line_ABC && counterclockwise_or_same_line_ABD)
            || (counterclockwise_or_same_line_ABC && clockwise_or_same_line_ABD);

        let clockwise_or_same_line_CDA = clockwise_CDA || same_straight_line_CDA;
        let counterclockwise_or_same_line_CDA = counterclockwise_CDA || same_straight_line_CDA;
        let clockwise_or_same_line_CDB = clockwise_CDB || same_straight_line_CDB;
        let counterclockwise_or_same_line_CDB = counterclockwise_CDB || same_straight_line_CDB;

        // 線分CDを充分に伸ばした場合、線分ABと交差するか。
        // 一方がCDに対して時計回りかつ、もう一方が反時計回りの場合は交差する
        let does_CD_sep_AB = (clockwise_or_same_line_CDA && counterclockwise_or_same_line_CDB)
            || (counterclockwise_or_same_line_CDA && clockwise_or_same_line_CDB);

        // 両方がtrueなら伸ばさずとも現在の長さで交差している
        does_AB_sep_CD && does_CD_sep_AB
    }
}
