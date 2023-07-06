use std::cmp;

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    let mut dp_table = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                dp_table[i][j] = cmp::max(
                    cmp::max(dp_table[i - 1][j], dp_table[i][j - 1]),
                    dp_table[i - 1][j - 1] + 1,
                );
            } else {
                dp_table[i][j] = cmp::max(dp_table[i - 1][j], dp_table[i][j - 1]);
            }
        }
    }
    println!("{}", dp_table[s.len()][t.len()]);
}
