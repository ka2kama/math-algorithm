#![allow(nonstandard_style)]

use proconio::derive_readable;
use proconio::input;

#[derive_readable]
#[derive(Debug)]
struct Item {
    weight: usize,
    value: usize,
}

fn main() {
    input! { N: usize, W: usize, A: [Item; N] };

    let mut max_value_of_nth_and_wth =
        vec![vec![0_usize; W + 1].into_boxed_slice(); N + 1].into_boxed_slice();

    for n in 1..=N {
        let Item { weight, value } = A[n - 1];
        for w in 1..=W {
            // 自分を入れる   -> n - 1個目, w - weightまでの最大価値 + value (w >= weight)
            // 自分を入れない -> n - 1個目, wまでの最大価値
            max_value_of_nth_and_wth[n][w] = if w >= weight {
                (max_value_of_nth_and_wth[n - 1][w - weight] + value)
                    .max(max_value_of_nth_and_wth[n - 1][w])
            } else {
                max_value_of_nth_and_wth[n - 1][w]
            }
        }
    }
    println!("{}", max_value_of_nth_and_wth[N][W]);
}
