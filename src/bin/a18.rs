use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n],
    }

    let mut dp_table = vec![vec![0; s + 1]; n];

    // 1番目
    for j in 0..s + 1 {
        if a[0] <= j {
            dp_table[0][j] = a[0];
        }
    }

    // 2番目以降
    for i in 1..n {
        for j in 0..s + 1 {
            if a[i] > j {
                dp_table[i][j] = dp_table[i - 1][j];
            } else {
                dp_table[i][j] = cmp::max(dp_table[i - 1][j - a[i]] + a[i], dp_table[i - 1][j]);
            }
        }
    }

    if dp_table[n - 1][s] == s {
        println!("{}", "Yes");
    } else {
        println!("{}", "No");
    }
}
