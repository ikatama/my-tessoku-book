use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }

    let mut count = 0;
    let mut right = 1;

    for left in 0..n {
        while right < n && a[right] - a[left] <= k {
            right += 1;
        }
        count += (right - 1) - left;
    }
    println!("{}", count);
}
