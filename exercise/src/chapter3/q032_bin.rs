#![allow(nonstandard_style)]

use num_traits::ToPrimitive;
use proconio::input;

fn main() {
    input! { N: usize, X: i64, mut A: [i64; N] };

    A.sort_unstable();

    println!(
        "{}",
        if BinarySearch::run(&A, X) {
            "Yes"
        } else {
            "No"
        }
    );
}

struct BinarySearch<'a, T: PartialOrd> {
    A: &'a [T],
    X: T,
}

impl<'a, T: PartialOrd> BinarySearch<'a, T> {
    fn run(A: &'a [T], X: T) -> bool {
        let bs = Self { A, X };
        bs.binary_search(0, A.len().to_isize().unwrap() - 1)
    }

    fn binary_search(&self, left: isize, right: isize) -> bool {
        if left > right {
            return false;
        }

        let mid = (left + right) / 2;
        if mid < 0 || mid as usize >= self.A.len() {
            return false;
        }

        if self.A[mid as usize] < self.X {
            self.binary_search(mid + 1, right)
        } else if self.A[mid as usize] > self.X {
            self.binary_search(left, mid - 1)
        } else {
            true
        }
    }
}
