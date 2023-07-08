use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b: [usize; n-1],
    }

    let mut dp = vec![0; n + 1];

    for i in 1..n {
        dp[a[i - 1]] = cmp::max(dp[a[i - 1]], dp[i] + 100);
        dp[b[i - 1]] = cmp::max(dp[b[i - 1]], dp[i] + 150);
    }

    println!("{}", dp[n]);
}
