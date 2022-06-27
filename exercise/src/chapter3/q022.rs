#![allow(nonstandard_style)]

use std::collections::HashMap;

use proconio::input;

const S: u64 = 100_000;
const HALF_S: u64 = S / 2;

fn main() {
    input! { N: usize, A: [u64; N] };
    let ans = solve(A);
    println!("{}", ans);
}

fn solve(A: Vec<u64>) -> usize {
    let map: HashMap<u64, usize> = {
        let mut map = HashMap::new();
        for v in A {
            *map.entry(v).or_insert(0) += 1;
        }
        map
    };

    let s1: usize = (1..HALF_S)
        .map(|i| (*map.get(&i).unwrap_or(&0), *map.get(&(S - i)).unwrap_or(&0)))
        .map(|(a, b)| a * b)
        .sum();

    let s2 = {
        let cnt_just_half = *map.get(&HALF_S).unwrap_or(&0);
        cnt_just_half * (cnt_just_half - 1) / 2
    };

    s1 + s2
}
