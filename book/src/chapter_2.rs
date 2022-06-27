#![allow(unused, nonstandard_style)]
#![cfg(test)]

mod ch_2_1 {
    use std::convert::TryFrom;
    use std::iter::FromIterator;

    use num_integer::Integer;

    fn q_2_1_2(a: i32, b: i32, c: i32) -> i32 {
        a + b + c
    }

    #[test]
    fn test_q_2_1_2() {
        assert_eq!(q_2_1_2(25, 4, 12), 41);
    }

    fn q_2_1_3(a1: i32, a2: i32, a3: i32) -> String {
        format!("{a1}{a2}{a3}")
    }

    #[test]
    fn test_q_2_1_3() {
        assert_eq!(q_2_1_3(1, 2, 8), "128");
    }

    fn q_2_1_4_1(n: &str) -> u32 {
        let digits = n.chars().map(|c| c.to_digit(10).unwrap());
        digits
            .rev()
            .enumerate()
            .map(|(i, x)| x * 2_u32.pow(u32::try_from(i).unwrap()))
            .sum()
    }

    fn q_2_1_4_2(n: u32) -> String {
        let mut m = n;
        let mut v: Vec<char> = Vec::new();
        while m != 0 {
            let (quot, rem) = m.div_rem(&2);
            v.push(char::from_digit(rem, 10).unwrap());
            m = quot;
        }

        String::from_iter(v)
    }

    #[test]
    fn test_q_2_1_4() {
        assert_eq!(q_2_1_4_1("1001"), 9);
        assert_eq!(q_2_1_4_2(9), "1001");
    }
}

mod ch_2_2 {
    use std::convert::TryFrom;

    use num_integer::Roots;

    fn q_2_2_1(n: i64) -> i32 {
        (n as f64).log10() as i32
    }

    fn q_2_2_2(n: i32) -> f64 {
        f64::try_from(n).unwrap().sqrt()
    }

    #[test]
    fn test_q_2_2_1() {
        assert_eq!(q_2_2_1(10_000), 4);
        assert_eq!(q_2_2_1(100_000_000), 8);
        assert_eq!(q_2_2_1(1_000_000_000_000), 12);
    }

    #[test]
    fn test_q_2_2_2() {
        assert_eq!(q_2_2_2(841), 29.);
        assert_eq!(29_i32.pow(2), 841);
        assert_eq!(1024.nth_root(5), 4);
        assert_eq!(4_i32.pow(5), 1024);
    }

    #[test]
    fn test_q_2_2_3() {
        assert_eq!(13 & 14, 12);
        assert_eq!(13 | 14, 15);
        assert_eq!(13 ^ 14, 3);
        assert_eq!(8 | 4 | 2 | 1, 15);
    }

    fn q_2_2_4(v: &[i32]) -> i32 {
        let s: i32 = v.iter().sum();
        s % 100
    }

    #[test]
    fn test_q_2_2_4() {
        assert_eq!(q_2_2_4(&vec![100, 201, 302]), 3);
        assert_eq!(q_2_2_4(&vec![100, 200, 300]), 0);
    }
}

mod ch_2_3 {
    use num_integer::Roots;

    fn q_2_3_1(n: i32) -> i32 {
        n.pow(3)
    }

    #[test]
    fn test_q_2_3_1() {
        assert_eq!(q_2_3_1(1), 1);
        assert_eq!(q_2_3_1(5), 125);
        assert_eq!(q_2_3_1(10), 1000);
    }

    fn q_2_3_2_3(n: f64) -> i64 {
        n.floor() as i64
    }

    fn q_2_3_2_4(n: f64) -> i64 {
        n.ceil() as i64
    }

    #[test]
    fn test_q_2_3_2() {
        assert_eq!(8_f64.log2(), 3.);
        assert_eq!(100_i64.pow(3).nth_root(2), 1000);
        assert_eq!(100_f64.powf(1.5), 1000.);
        assert_eq!(q_2_3_2_3(20.21), 20);
        assert_eq!(q_2_3_2_3(-2.1), -3);
        assert_eq!(q_2_3_2_4(20.21), 21);
        assert_eq!(q_2_3_2_4(-2.1), -2);
    }

    #[test]
    fn test_q_2_3_4() {
        assert_eq!(2_i32.pow(20), 1048576);
    }

    #[test]
    fn test_q_2_3_5() {
        assert_eq!(1_000_000_f64.log10(), 6.);
    }

    fn q_2_3_6(a: f64, b: f64) -> f64 {
        32_f64.powf(a - b).ceil()
    }

    #[test]
    fn test_q_2_3_6() {
        assert_eq!(q_2_3_6(6., 5.), 32.);
        assert_eq!(q_2_3_6(7.3, 5.3), 1024.);
        assert_eq!(q_2_3_6(9.0, 7.2), 512.);
    }

    fn q_2_3_7(x: u64) -> usize {
        (x as f64).log2().ceil() as usize
    }

    #[test]
    fn test_q_2_3_7() {
        assert_eq!(q_2_3_7(7), 3);
        assert_eq!(q_2_3_7(10), 4);
    }
}
