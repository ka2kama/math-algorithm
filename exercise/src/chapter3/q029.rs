#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: usize, };

    let mut how_many_ways_of_ith = vec![0_i64; N + 1];
    how_many_ways_of_ith[0] = 1;
    how_many_ways_of_ith[1] = 1;
    for i in 2..=N {
        how_many_ways_of_ith[i] = how_many_ways_of_ith[i - 1] + how_many_ways_of_ith[i - 2]
    }
    println!("{}", how_many_ways_of_ith[N]);
}
