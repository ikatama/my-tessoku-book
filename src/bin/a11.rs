use proconio::input;

fn main() {
    // 2分探索で解く
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut left = 0;
    let mut right = n;

    while left < right {
        let mid = (left + right) / 2;

        if a[mid] == x {
            // 配列のインデックスは0から始まるため、出力するときに+1
            println!("{}", mid + 1);
            break;
        } else if x < a[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
}
