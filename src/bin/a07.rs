use std::vec;

use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
    }

    let mut v: Vec<i32> = vec![0; d];

    for _ in 0..n {
        input! {
            l: usize,
            r: usize,
        }

        for i in l..=r {
            v[i - 1] += 1;
        }
    }

    for e in v {
        println!("{}", e);
    }
}
