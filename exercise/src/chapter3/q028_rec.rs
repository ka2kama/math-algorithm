#![allow(nonstandard_style)]

use proconio::input;

fn main() {
    input! { N: usize, H: [i64; N] };

    let ans = Solver::solve(H, N);
    println!("{}", ans);
}

struct Solver {
    H: Box<[i64]>,
    memo: Box<[Option<i64>]>,
}

impl Solver {
    fn solve(H: Vec<i64>, N: usize) -> i64 {
        let mut s = Self {
            H: H.into_boxed_slice(),
            memo: vec![None; N + 1].into_boxed_slice(),
        };
        s.min_cost_of(N)
    }

    fn min_cost_of(&mut self, n: usize) -> i64 {
        if let Some(nth_min_cost) = self.memo[n] {
            nth_min_cost
        } else {
            let H = &self.H;
            let nth_min_cost = if n == 1 {
                // 最初の島への移動コストは0
                0
            } else if n == 2 {
                // 2つ目の島への移動経路は1通りのみ
                (H[1] - H[0]).abs()
            } else {
                // 3つ目以降の島への最小移動コスト
                let h_idx = n - 1;
                let current_height = H[h_idx];
                let prev_height = H[h_idx - 1];
                let two_prev_height = H[h_idx - 2];

                let from_prev = self.min_cost_of(n - 1) + (current_height - prev_height).abs();
                let from_two_prev =
                    self.min_cost_of(n - 2) + (current_height - two_prev_height).abs();
                from_prev.min(from_two_prev)
            };
            self.memo[n] = Some(nth_min_cost);
            nth_min_cost
        }
    }
}
