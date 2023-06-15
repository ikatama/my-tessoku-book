use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let sorted_a: Vec<usize> = {
        let mut b: Vec<usize> = a.clone();
        b.sort();
        b.into_iter().unique().collect_vec()
    };

    let mut compressed: Vec<usize> = vec![0; n];

    for i in 0..n {
        let mut left = 0;
        let mut right = sorted_a.len();
        while left < right {
            let mid = (left + right) / 2;
            if a[i] == sorted_a[mid] {
                compressed[i] = mid;
                break;
            } else if a[i] < sorted_a[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
    }

    println!(
        "{}",
        compressed.iter().map(|x| (x + 1).to_string()).join(" ")
    );
}
