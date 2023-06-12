use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
    }

    let mut l_max = vec![0; n];
    let mut r_max = vec![0; n];

    l_max[0] = a[0];
    r_max[n - 1] = a[n - 1];

    // 左から数えてi番目までの要素のうち最大の値をl_max[i]に格納する。
    for i in 1..n {
        l_max[i] = cmp::max(l_max[i - 1], a[i]);
    }

    for i in (0..(n - 1)).rev() {
        r_max[i] = cmp::max(r_max[i + 1], a[i]);
    }

    for _ in 0..d {
        input! {
            l: usize,
            r: usize,
        }

        let result = cmp::max(l_max[l - 2], r_max[r]);
        println!("{}", result);
    }
}
