use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        w: usize,
        w_v: [(usize, usize); n],
    }

    let mut dp_table = vec![vec![0; w + 1]; n];

    // 1番目
    for j in 0..=w {
        if w_v[0].0 <= j {
            dp_table[0][j] = w_v[0].1;
        }
    }

    // 2番目以降
    for i in 1..n {
        for j in 0..=w {
            if w_v[i].0 > j {
                dp_table[i][j] = dp_table[i - 1][j];
            } else {
                dp_table[i][j] =
                    cmp::max(dp_table[i - 1][j - w_v[i].0] + w_v[i].1, dp_table[i - 1][j]);
            }
        }
    }
    println!("{}", dp_table[n - 1][w]);
}
