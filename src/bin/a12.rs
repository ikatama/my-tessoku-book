use proconio::input;

fn main() {
    // 二分探索を用いない実装
    // TLEになる想定
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    // k枚未満印刷する秒数
    let mut low = 0;
    // k枚以上印刷する秒数
    let mut high = 100_000_000;

    while low < high {
        let mid = (low + high) / 2;

        let mut printed = 0;
        for e in &a {
            printed += mid / e;
        }

        if printed >= k {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    println!("{}", low)
}
