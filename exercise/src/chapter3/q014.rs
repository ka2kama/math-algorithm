#![allow(nonstandard_style)]

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! { N: u64, };
    let prime_factors = get_prime_factors(N);
    println!(
        "{}",
        prime_factors
            .into_iter()
            .flat_map(|(i, cnt)| itertools::repeat_n(i, cnt))
            .join(" ")
    );
}

fn get_prime_factors(N: u64) -> Vec<(u64, usize)> {
    let divisors = (2_u64..)
        .take_while(|&i| i.pow(2) <= N)
        .filter(|&i| N % i == 0);

    let mut rest = N;
    let mut prime_factors = Vec::new();
    for i in divisors {
        if rest == 1 {
            break;
        }
        let mut cnt = 0_usize;
        while rest % i == 0 {
            cnt += 1;
            rest /= i;
        }
        if cnt > 0 {
            prime_factors.push((i, cnt));
        }
    }
    if rest >= 2 {
        prime_factors.push((rest, 1));
    }
    prime_factors
}
