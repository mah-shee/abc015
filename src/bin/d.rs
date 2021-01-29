#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::{max, min};
#[fastout]
fn main() {
    input! {
        w: usize,
        n: usize,
        k: usize,
        ss: [(usize, usize); n],
    }
    let mut dp = vec![vec![std::isize::MAX; 100 * n + 1]; k + 1];
    dp[0][0] = 0;

    for (a, b) in ss {
        for prev_k in (0..k).rev() {
            for prev_v in (0..100 * n).rev() {
                if prev_v + b > 100 * n {
                    continue;
                }
                if dp[prev_k][prev_v] == std::isize::MAX {
                    continue;
                }
                dp[prev_k + 1][prev_v + b] =
                    min(dp[prev_k + 1][prev_v + b], dp[prev_k][prev_v] + a as isize);
            }
        }
    }
    let mut ans = 0;
    for k in 0..k + 1 {
        for v in 0..100 * n + 1 {
            if dp[k][v] <= w as isize {
                ans = max(ans, v);
            }
        }
    }
    println!("{}", ans);
}
