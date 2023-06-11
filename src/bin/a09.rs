use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let mut table: Vec<Vec<i32>> = vec![vec![0; w + 1]; h + 1];

    // 重みの加算
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }
        table[a - 1][b - 1] += 1;
        table[a - 1][d] -= 1;
        table[c][b - 1] -= 1;
        table[c][d] += 1;
    }

    // 横方向の累積和
    for y in 0..h {
        for x in 1..w {
            table[y][x] += table[y][x - 1];
        }
    }

    // 縦方向の累積和
    for y in 1..h {
        for x in 0..w {
            table[y][x] += table[y - 1][x];
        }
    }

    for row in table[0..h].iter() {
        let row: Vec<String> = row[0..w].iter().map(|x| format!("{}", x)).collect();
        println!("{}", row.join(" "));
    }
}
