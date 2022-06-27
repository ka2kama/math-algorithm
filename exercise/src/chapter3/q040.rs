#![allow(nonstandard_style)]

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, A: [usize; N - 1], M: usize, B: [usize; M] };

    let ans = solve(A, B);
    println!("{}", ans);
}

fn solve(A: Vec<usize>, B: Vec<usize>) -> usize {
    let mut accum = vec![0; A.len() + 1];
    for (i, x) in A.into_iter().enumerate() {
        accum[i + 1] = accum[i] + x;
    }

    let mut s = 0;
    for (&current, &next) in B.iter().zip(B.iter().skip(1)) {
        let (a, b) = if current < next {
            (current, next)
        } else {
            (next, current)
        };
        s += accum[b - 1] - accum[a - 1];
    }

    s
}
