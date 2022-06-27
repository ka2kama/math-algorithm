#![allow(unused, nonstandard_style)]
#![cfg(test)]

mod ch_3_1 {
    fn is_prime(n: u64) -> bool {
        (2_u64..).take_while(|&i| i.pow(2) <= n).all(|i| n % i != 0)
    }

    #[test]
    fn test_q_3_1_1() {
        assert_eq!(is_prime(33), false);
    }

    fn q_3_1_2(n: u64) -> Vec<u64> {
        let m = n;
        let v = Vec::new();
        for i in (2_u64..).take_while(|i| i.pow(2) <= n) {}
        v
    }
}

mod ch_3_2 {
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        println!("{}, {}", a, b);
        while a >= 1 && b >= 1 {
            if a < b {
                b = b % a;
            } else {
                a = a % b;
            }
            println!("{}, {}", a, b);
        }
        if a >= 1 {
            a
        } else {
            b
        }
    }

    #[test]
    fn test_q_3_2_1() {
        assert_eq!(gcd(6, 8), 2);
        assert_eq!(gcd(372, 506), 2);
    }

    fn q_3_2_2(v: &[i64]) -> i64 {
        v[2..].iter().fold(gcd(v[0], v[1]), |a, &b| gcd(a, b))
    }

    #[test]
    fn test_q_3_2_2() {
        assert_eq!(q_3_2_2(&vec![24, 40, 60, 80, 90, 120]), 2);
    }

    fn lcm(a: i64, b: i64) -> i64 {
        a / gcd(a, b) * b
    }

    fn q_3_2_3(v: &[i64]) -> i64 {
        v[2..].iter().fold(lcm(v[0], v[1]), |a, &b| lcm(a, b))
    }

    #[test]
    fn test_q_3_2_3() {
        assert_eq!(q_3_2_3(&vec![24, 40, 60, 80, 90, 120]), 720);
    }
}

mod ch_3_3 {
    use itertools::Itertools;

    #[test]
    fn test_q_3_3_1() {
        assert_eq!((0..2).combinations(1).count(), 2);
        assert_eq!((0..8).combinations(5).count(), 56);
        assert_eq!((0..7).permutations(2).count(), 42);
        assert_eq!((0..10).permutations(3).count(), 720);
    }

    #[test]
    fn test_q_3_3_2() {
        assert_eq!(3 * 5 * 2, 30);
    }

    fn q_3_3_3(n: usize, r: usize) -> usize {
        (0..n).combinations(r).count()
    }

    fn q_3_3_4(v: &[usize]) -> usize {
        let (mut cnt_100, mut cnt_200, mut cnt_300, mut cnt_400) =
            (0_usize, 0_usize, 0_usize, 0_usize);

        for x in v {
            match x {
                &100 => cnt_100 += 1,
                &200 => cnt_200 += 1,
                &300 => cnt_300 += 1,
                &400 => cnt_400 += 1,
                _ => {}
            }
        }

        cnt_100 * cnt_400 + cnt_200 * cnt_300
    }

    #[test]
    fn test_q_3_3_4() {
        assert_eq!(
            q_3_3_4(&[100, 100, 100, 100, 200, 200, 300, 300, 300, 400, 400, 400, 400]),
            22
        );
    }

    fn q_3_3_5(v: &[usize]) -> usize {
        let (mut red, mut yellow, mut blue) = (0_usize, 0_usize, 0_usize);
        for x in v {
            match x {
                &1 => red += 1,
                &2 => yellow += 1,
                &3 => blue += 1,
                _ => {}
            }
        }

        q_3_3_3(red, 2) + q_3_3_3(yellow, 2) + q_3_3_3(blue, 2)
    }

    #[test]
    fn test_q_3_3_5() {
        assert_eq!(q_3_3_5(&[1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3]), 15);
    }

    fn q_3_3_6(v: &[usize]) -> usize {
        v.iter()
            .combinations(2)
            .filter(|c| c[0] + c[1] == 100000)
            .count()
    }

    #[test]
    fn test_q_3_3_6() {
        assert_eq!(q_3_3_6(&[20000, 80000, 50000, 50000, 30000]), 2);
    }
}

mod ch_3_4 {
    fn q_3_4_2(v: &[(usize, f64)]) -> f64 {
        v.iter().fold(0., |acc, &(m, p)| acc + (m as f64) * p)
    }

    #[test]
    fn test_q_3_4_2() {
        let v = [
            (1_000_000, 1. / 10000.),
            (100_000, 9. / 10000.),
            (10_000, 9. / 1000.),
            (1000, 9. / 100.),
            (0, 9. / 10.),
        ];
        assert_eq!(q_3_4_2(&v), 370.);
    }
}

mod ch_3_6 {
    struct MergeSort {
        v: Vec<i64>,
    }

    impl MergeSort {
        fn sort(&mut self, l: usize, r: usize) {
            if r - l == 1 {
                return;
            }

            let m = (l + r) / 2;
            self.sort(l, m);
            self.sort(m, r);

            let c = self.merge(l, r, m);

            for (i, x) in c.into_iter().enumerate() {
                self.v[l + i] = x;
            }
        }

        fn merge(&self, l: usize, r: usize, m: usize) -> Vec<i64> {
            let mut c = Vec::new();
            let (mut c1, mut c2) = (l, m);
            while c1 != m || c2 != r {
                if c1 == m {
                    c.push(self.v[c2]);
                    c2 += 1;
                } else if c2 == r {
                    c.push(self.v[c1]);
                    c1 += 1;
                } else {
                    if self.v[c1] < self.v[c2] {
                        c.push(self.v[c1]);
                        c1 += 1;
                    } else {
                        c.push(self.v[c2]);
                        c2 += 1;
                    }
                }
            }
            c
        }
    }

    #[test]
    fn test_q_3_6_3() {
        let v = vec![1, 4, 3, 2];
        let mut g = MergeSort { v };
        g.sort(0, g.v.len());
        assert_eq!(g.v, [1, 2, 3, 4]);
    }
}

mod ch_3_7 {
    use std::cmp::{max, min};

    fn code_3_7_1(島ごとの高さ: &[i32]) -> i32 {
        let ジャンプ時の体力消費量取得 = |ジャンプ元の島: usize, ジャンプ先の島: usize| -> i32 {
            let ジャンプ元の島の高さ = 島ごとの高さ[ジャンプ元の島];
            let ジャンプ先の島の高さ = 島ごとの高さ[ジャンプ先の島];

            (ジャンプ元の島の高さ - ジャンプ先の島の高さ).abs()
        };
        let 島の数 = 島ごとの高さ.len();
        let mut 最初の島からそれぞれの島に行くための最小体力消費量 =
            vec![0; 島の数].into_boxed_slice();
        let 最初の島 = 0_usize;
        let 二つ目の島 = 1_usize;
        let 三つ目の島 = 2_usize;
        最初の島からそれぞれの島に行くための最小体力消費量[最初の島] = 0;
        最初の島からそれぞれの島に行くための最小体力消費量[二つ目の島] =
            ジャンプ時の体力消費量取得(最初の島, 二つ目の島);

        for 今いる島 in 三つ目の島..島の数 {
            let 一つ前の島からジャンプした場合の_最初の島から今いる島へたどり着くための合計体力消費量 = {
                let 一つ前の島 = 今いる島 - 1;
                let 一つ前の島にたどり着いたときの最小体力消費量 =
                    最初の島からそれぞれの島に行くための最小体力消費量[一つ前の島];
                一つ前の島にたどり着いたときの最小体力消費量
                    + ジャンプ時の体力消費量取得(一つ前の島, 今いる島)
            };
            let 二つ前の島からジャンプした場合の_最初の島から今いる島へたどり着くための合計体力消費量 = {
                let 二つ前の島 = 今いる島 - 2;
                let 二つ前の島にたどり着いたときの最小体力消費量 =
                    最初の島からそれぞれの島に行くための最小体力消費量[二つ前の島];
                二つ前の島にたどり着いたときの最小体力消費量
                    + ジャンプ時の体力消費量取得(二つ前の島, 今いる島)
            };
            最初の島からそれぞれの島に行くための最小体力消費量[今いる島] = min(
                一つ前の島からジャンプした場合の_最初の島から今いる島へたどり着くための合計体力消費量,
                二つ前の島からジャンプした場合の_最初の島から今いる島へたどり着くための合計体力消費量,
            );
        }
        let 最後の島 = 島の数 - 1;
        最初の島からそれぞれの島に行くための最小体力消費量[最後の島]
    }

    #[test]
    fn test_code_3_7_1() -> () {
        let v = [8, 6, 9, 2, 1];
        assert_eq!(code_3_7_1(&v), 7);
    }

    struct 品物 {
        重さ: usize,
        価値: i64,
    }

    fn ナップサック問題(すべての品物: &[品物], 最大積載重量: usize) -> i64 {
        let 品物の数 = すべての品物.len();
        let mut i個目の品物まで選んだ場合の_積載重量ごとの価値の最大値 =
            vec![vec![0; 最大積載重量 + 1].into_boxed_slice(); 品物の数 + 1].into_boxed_slice();

        for (index, 現在の品物) in すべての品物.iter().enumerate() {
            let 品物i個目 = index + 1;
            for 積載重量 in 0..=最大積載重量 {
                let そもそも現在の積載重量では現在の品物を詰めることができない場合 =
                    積載重量 < 現在の品物.重さ;

                let 一つ前までの品物から選んだ場合の_現在と同じ積載重量での価値の最大値 =
                    i個目の品物まで選んだ場合の_積載重量ごとの価値の最大値[品物i個目 - 1][積載重量];

                let 現在の品物まで選んだ場合の_現在の積載重量における価値の最大値 =
                    if そもそも現在の積載重量では現在の品物を詰めることができない場合 {
                        一つ前までの品物から選んだ場合の_現在と同じ積載重量での価値の最大値
                    } else {
                        let 現在の品物を詰めない場合の価値の最大値 = 一つ前までの品物から選んだ場合の_現在と同じ積載重量での価値の最大値;
                        let 現在の品物を詰める場合の価値の最大値 = {
                            let 現在の品物を詰める場合の_現在の積載重量の残り = 積載重量 - 現在の品物.重さ;
                            let 一つ前までの品物から選ぶ場合の_上記の積載重量内での価値の最大値 =
                                i個目の品物まで選んだ場合の_積載重量ごとの価値の最大値[品物i個目 - 1][現在の品物を詰める場合の_現在の積載重量の残り];
                            一つ前までの品物から選ぶ場合の_上記の積載重量内での価値の最大値 + 現在の品物.価値
                        };
                        max(現在の品物を詰めない場合の価値の最大値, 現在の品物を詰める場合の価値の最大値)
                    };

                i個目の品物まで選んだ場合の_積載重量ごとの価値の最大値[品物i個目][積載重量] =
                    現在の品物まで選んだ場合の_現在の積載重量における価値の最大値;
            }
        }

        i個目の品物まで選んだ場合の_積載重量ごとの価値の最大値[品物の数][最大積載重量]
    }

    #[test]
    fn test_ナップサック問題() {
        let すべての品物 = vec![
            品物 {
                重さ: 3, 価値: 100
            },
            品物 {
                重さ: 6, 価値: 210
            },
            品物 {
                重さ: 4, 価値: 130
            },
            品物 {
                重さ: 2, 価値: 57
            },
        ];
        assert_eq!(ナップサック問題(&すべての品物, 10), 340);
    }

    fn q_3_7_4(すべての整数: &[usize], 最終合計: usize) -> bool {
        if 最終合計 == 0 {
            return true;
        }

        let 整数の数 = すべての整数.len();
        let mut i番目までの整数で_0から最終合計までの値を作れるか =
            vec![vec![false; 最終合計 + 1].into_boxed_slice(); 整数の数 + 1].into_boxed_slice();

        // 合計0は、何も選ばなければいいだけなので必ず作れる
        for i番目までの整数で各種合計値を作れるか in
            i番目までの整数で_0から最終合計までの値を作れるか.iter_mut()
        {
            i番目までの整数で各種合計値を作れるか[0] = true;
        }

        for i in 1..=整数の数 {
            let i番目の整数 = すべての整数[i - 1];
            for 調べたい合計 in 1..=最終合計 {
                let i番目より前の段階で調べたい合計を作れる =
                    i番目までの整数で_0から最終合計までの値を作れるか[i - 1][調べたい合計];
                if i番目より前の段階で調べたい合計を作れる {
                    i番目までの整数で_0から最終合計までの値を作れるか[i][調べたい合計] = true;
                } else {
                    i番目までの整数で_0から最終合計までの値を作れるか[i][調べたい合計] =
                        i番目の整数 <= 調べたい合計 && i番目までの整数で_0から最終合計までの値を作れるか[i - 1][調べたい合計 - i番目の整数];
                }
            }
        }

        i番目までの整数で_0から最終合計までの値を作れるか[整数の数][最終合計]
    }

    #[test]
    fn test_q_3_7_4() {
        assert_eq!(q_3_7_4(&[1, 3, 7, 10, 13], 21), true);
        assert_eq!(q_3_7_4(&[2, 4, 6, 8, 10], 19), false);
    }
}
