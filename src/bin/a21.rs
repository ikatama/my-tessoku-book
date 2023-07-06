use proconio::input;

fn main() {
    input! {
        n: usize,
        p_a: [(usize, usize); n],
    }

    let mut dp_table = vec![vec![0; n]; n];

    for i in (0..(n - 1)).rev() {
        for j in (i + 1)..n {
            let a = if p_a[i].0 - 1 > i && p_a[i].0 - 1 <= j {
                p_a[i].1
            } else {
                0
            };
            let b = if p_a[j].0 - 1 >= i && p_a[j].0 - 1 < j {
                p_a[j].1
            } else {
                0
            };
            dp_table[i][j] = std::cmp::max(dp_table[i + 1][j] + a, dp_table[i][j - 1] + b);
        }
    }

    println!("{}", dp_table[0][n - 1]);
}
