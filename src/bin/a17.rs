use std::cmp;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
        b:[usize;n-2],
    }

    let mut dp = vec![0; n];

    dp[0] = 0;
    dp[1] = a[0];

    for i in 2..n {
        dp[i] = cmp::min(dp[i - 1] + a[i - 1], dp[i - 2] + b[i - 2]);
    }

    let mut answer = vec![n];
    let mut i = n - 1;
    while i >= 1 {
        if dp[i] == dp[i - 1] + a[i - 1] {
            answer.push(i);
            i -= 1;
        } else {
            answer.push(i - 1);
            i -= 2;
        }
    }

    let answer = answer.iter().map(|x| x.to_string()).rev().collect_vec();

    println!("{}", answer.len());
    println!("{}", answer.join(" "));
}
