#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: usize, A: [usize; N] };

    let mut max_capability_of_nth = vec![vec![0_usize; 2]; N + 1].into_boxed_slice();
    max_capability_of_nth[1][1] = A[0];
    for n in 2..=N {
        // n日目に勉強しなかった場合の最大実力
        max_capability_of_nth[n][0] =
            max_capability_of_nth[n - 1][0].max(max_capability_of_nth[n - 1][1]);
        // n日目に勉強した場合の最大実力
        max_capability_of_nth[n][1] = max_capability_of_nth[n - 1][0] + A[n - 1];
    }
    println!(
        "{}",
        max_capability_of_nth[N][0].max(max_capability_of_nth[N][1])
    );
}
