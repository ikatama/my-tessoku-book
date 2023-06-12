use proconio::input;

fn main() {
    // 二分探索を用いない実装
    // TLEになる想定
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut sec = 0;

    let mut printed = 0;
    while printed < k {
        sec += 1;
        printed = 0;
        for printer in &a {
            printed += sec / printer;
        }
    }
    println!("{}", sec)
}
