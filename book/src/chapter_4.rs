#![allow(unused, nonstandard_style)]
#![cfg(test)]

mod ch_4_1 {
    use std::ptr::addr_of;

    use itertools::Itertools;
    use num_traits::cast::ToPrimitive;
    use num_traits::Pow;
    use pretty_assertions::assert_eq;
    use pretty_assertions::assert_ne;

    use crate::chapter_4::ch_4_1::パターン::*;

    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

    #[derive(Debug, Eq, PartialEq)]
    struct 点 {
        x: i64,
        y: i64,
    }

    #[derive(Debug, Eq, PartialEq)]
    struct 成分表示 {
        x: i64,
        y: i64,
    }

    enum パターン {
        角ABCが90度より大きい,
        角ABC_角ACBがともに90度以下,
        角ACBが90度より大きい,
    }

    fn code_4_1_1(点A: 点, 点B: 点, 点C: 点) -> f64 {
        let ベクトルBAの成分表示 = 成分表示 {
            x: 点A.x - 点B.x,
            y: 点A.y - 点B.y,
        };
        let ベクトルBCの成分表示 = 成分表示 {
            x: 点C.x - 点B.x,
            y: 点C.y - 点B.y,
        };
        let ベクトルCAの成分表示 = 成分表示 {
            x: 点A.x - 点C.x,
            y: 点A.y - 点C.y,
        };
        let ベクトルCBの成分表示 = 成分表示 {
            x: 点B.x - 点C.x,
            y: 点B.y - 点C.y,
        };

        let ベクトルBAとBCの内積 = ベクトルBAの成分表示.x * ベクトルBCの成分表示.x
            + ベクトルBAの成分表示.y * ベクトルBCの成分表示.y;

        let ベクトルCAとCBの内積 = ベクトルCAの成分表示.x * ベクトルCBの成分表示.x
            + ベクトルCAの成分表示.y * ベクトルCBの成分表示.y;

        let pattern = if ベクトルBAとBCの内積 < 0 {
            角ABCが90度より大きい
        } else if ベクトルCAとCBの内積 < 0 {
            角ACBが90度より大きい
        } else {
            角ABC_角ACBがともに90度以下
        };

        let 点Aと線分B上の最短距離 = match pattern {
            角ABCが90度より大きい => {
                ((ベクトルBAの成分表示.x.pow(2) + ベクトルBAの成分表示.y.pow(2)) as f64).sqrt()
            }
            角ACBが90度より大きい => {
                ((ベクトルCAの成分表示.x.pow(2) + ベクトルCAの成分表示.y.pow(2)) as f64).sqrt()
            }
            角ABC_角ACBがともに90度以下 => {
                let ベクトルBAとBCの外積 = (ベクトルBAの成分表示.x * ベクトルBCの成分表示.y
                    - ベクトルBAの成分表示.y * ベクトルBCの成分表示.x)
                    .abs();
                let ベクトルBAとBCが作る平行四辺形の面積 = ベクトルBAとBCの外積;
                let BCの長さ =
                    ((ベクトルBCの成分表示.x.pow(2) + ベクトルBCの成分表示.y.pow(2)) as f64).sqrt();

                ベクトルBAとBCが作る平行四辺形の面積 as f64 / BCの長さ
            }
        };

        点Aと線分B上の最短距離
    }

    #[test]
    fn test_code_4_4_1() -> Result<(), Error> {
        assert_eq!(
            code_4_1_1(点 { x: 3, y: 3 }, 点 { x: 2, y: 1 }, 点 { x: 6, y: 4 }),
            1.
        );
        Ok(())
    }

    fn ベクトルの大きさ(ベクトル: &成分表示) -> f64 {
        (ベクトル.x.pow(2) + ベクトル.y.pow(2))
            .to_f64()
            .unwrap()
            .sqrt()
    }

    fn q_4_1_1_1(
        ベクトルA: &成分表示,
        ベクトルB: &成分表示,
    ) -> Result<(f64, f64, 成分表示), Error> {
        let ベクトルAの大きさ = ベクトルの大きさ(ベクトルA);
        let ベクトルBの大きさ = ベクトルの大きさ(ベクトルB);
        let ベクトルAとBの和 = 成分表示 {
            x: ベクトルA.x + ベクトルB.x,
            y: ベクトルA.y + ベクトルB.y,
        };
        Ok((ベクトルAの大きさ, ベクトルBの大きさ, ベクトルAとBの和))
    }

    fn q_4_1_1_2(ベクトルA: &成分表示, ベクトルB: &成分表示) -> Result<i64, Error> {
        let ベクトルAとBの内積 = ベクトルA.x * ベクトルB.x + ベクトルA.y * ベクトルB.y;
        Ok(ベクトルAとBの内積)
    }

    fn q_4_1_1_3(
        ベクトルA: &成分表示, ベクトルB: &成分表示
    ) -> Result<bool, Error> {
        let ベクトルAとBの内積 = q_4_1_1_2(ベクトルA, ベクトルB)?;
        Ok(ベクトルAとBの内積 < 0)
    }

    fn q_4_1_1_4(ベクトルA: &成分表示, ベクトルB: &成分表示) -> Result<i64, Error> {
        let ベクトルAとBの外積 = (ベクトルA.x * ベクトルB.y - ベクトルA.y * ベクトルB.x).abs();
        Ok(ベクトルAとBの外積)
    }

    #[test]
    fn a_q_4_1_1() -> Result<(), Error> {
        let ベクトルA = 成分表示 { x: 2, y: 4 };
        let ベクトルB = 成分表示 { x: 3, y: -9 };
        assert_eq!(
            q_4_1_1_1(&ベクトルA, &ベクトルB)?,
            (20_f64.sqrt(), 90_f64.sqrt(), 成分表示 { x: 5, y: -5 })
        );
        assert_eq!(q_4_1_1_2(&ベクトルA, &ベクトルB)?, -30);
        assert_eq!(q_4_1_1_3(&ベクトルA, &ベクトルB)?, true);
        assert_eq!(q_4_1_1_4(&ベクトルA, &ベクトルB)?, 30);
        Ok(())
    }

    fn 二点間の距離(点A: &点, 点B: &点) -> f64 {
        ベクトルの大きさ(&成分表示 {
            x: 点A.x - 点B.x,
            y: 点A.y - 点B.y,
        })
    }

    fn q_4_1_2(すべての点: &[点]) -> f64 {
        let 最も近い2つの点の距離 = すべての点
            .iter()
            .combinations(2)
            .map(|c| 二点間の距離(c[0], c[1]))
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        (最も近い2つの点の距離 * 100000000.).floor() / 100000000.
    }

    #[test]
    fn a_q_4_1_2() -> Result<(), Error> {
        let xs = vec![点 { x: 0, y: 0 }, 点 { x: 2, y: 0 }, 点 { x: 1, y: 1 }];
        assert_eq!(q_4_1_2(&xs), 1.41421356);
        Ok(())
    }
}

mod ch_4_2 {
    use anyhow::Result;

    fn code_4_2_1(i日目までのそれぞれの来場者数: &[usize]) -> Vec<usize> {
        let mut i日目までの来場者数の累積和 =
            vec![0_usize; i日目までのそれぞれの来場者数.len()];
        for (i, 来場者数) in i日目までのそれぞれの来場者数.iter().enumerate().skip(1)
        {
            i日目までの来場者数の累積和[i] = i日目までの来場者数の累積和[i - 1] + 来場者数;
        }
        i日目までの来場者数の累積和
    }
}
