#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: usize, S: usize, A: [usize; N]};

    let can_make_X_from_up_to_mth = partial_sum(&A, S);

    let ans = if can_make_X_from_up_to_mth[N][S] {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}

fn partial_sum(A: &[usize], S: usize) -> Vec<Vec<bool>> {
    // m番目までの数字で、合計X (X = 0..=S) を作れるか
    let mut can_make_X_from_up_to_mth = vec![vec![false; S + 1]; A.len() + 1];

    // 合計0は、何も選ばなければいいだけなので必ず作れる
    for m in 0..can_make_X_from_up_to_mth.len() {
        can_make_X_from_up_to_mth[m][0] = true;
    }

    for (i, &a) in A.iter().enumerate() {
        let m = i + 1;
        for X in 1..=S {
            can_make_X_from_up_to_mth[m][X] = can_make_X_from_up_to_mth[m - 1][X]
                || (X >= a && can_make_X_from_up_to_mth[m - 1][X - a]);
        }
    }
    can_make_X_from_up_to_mth
}
