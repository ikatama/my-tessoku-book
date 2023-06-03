use proconio::input;

fn main() {
    input! {
        n: i32,
        q: i32,
        a: [i32;n],
    }

    for _ in 1..=q {
        input! {
            l: usize,
            r: usize,
        }
        let sum = a[l - 1..r].iter().fold(0, |sum, i| sum + i);
        println!("{}", sum);
    }
}
