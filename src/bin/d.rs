#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::cmp::max;
#[fastout]
fn main() {
    input! {
        w: usize,
        n: usize,
        k: usize,
        ss: [(usize, usize); n],
    }
    let mut dp = vec![vec![vec![-1isize; w + 1]; n + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        for j in 0..=k {
            for l in 0..=w {
                if dp[i][j][l] == -1 {
                    continue;
                }
                if j == k {
                    dp[i + 1][j][l] = max(dp[i + 1][j][l], dp[i][j][l]);
                } else {
                    if l + ss[i].0 <= w {
                        dp[i + 1][j + 1][l + ss[i].0 as usize] = max(
                            dp[i + 1][j + 1][l + ss[i].0],
                            dp[i][j][l] + ss[i].1 as isize,
                        );
                        dp[i + 1][j][l] = max(dp[i + 1][j][l], dp[i][j][l]);
                    } else {
                        dp[i + 1][j][l] = max(dp[i + 1][j][l], dp[i][j][l]);
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..w + 1 {
        for j in 0..n + 1 {
            ans = max(ans, dp[n][j][i]);
        }
    }
    println!("{}", ans);
}
