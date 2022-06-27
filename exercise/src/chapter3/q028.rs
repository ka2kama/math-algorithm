#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: usize, H: [i64; N] };

    let mut min_cost_of_ith = vec![0_i64; N + 1];
    min_cost_of_ith[2] = (H[1] - H[0]).abs();
    for i in 3..=N {
        let h_idx = i - 1;
        let current_height = H[h_idx];
        let prev_height = H[h_idx - 1];
        let two_prev_height = H[h_idx - 2];
        min_cost_of_ith[i] = (min_cost_of_ith[i - 1] + (current_height - prev_height).abs())
            .min(min_cost_of_ith[i - 2] + (current_height - two_prev_height).abs())
    }
    println!("{}", min_cost_of_ith[N]);
}
