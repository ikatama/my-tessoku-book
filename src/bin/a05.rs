use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
    }

    let mut count = 0;
    for a in 1..=n {
        for b in 1..=n {
            if a + b < k && a + b >= k - n {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
