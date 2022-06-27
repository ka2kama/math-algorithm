#![allow(nonstandard_style)]

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: usize, A: [u64; N] };

    let mut cnt = 0_u64;
    for i in 0..N {
        for j in i + 1..N {
            for k in j + 1..N {
                for l in k + 1..N {
                    for m in l + 1..N {
                        if A[i] + A[j] + A[k] + A[l] + A[m] == 1000 {
                            cnt += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", cnt);
}

#[fastout]
fn main2() {
    input! { N: usize, A: [u64; N] };
    let ans = A
        .into_iter()
        .combinations(5)
        .filter(|v| v.iter().sum::<u64>() == 1000)
        .count();
    println!("{}", ans);
}
