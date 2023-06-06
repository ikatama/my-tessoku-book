use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
    }

    for _ in 0..q {
        input! {
            a: usize,
            b: usize,
            c: usize,
            d: usize,
        }

        let mut sum: usize = 0;
        for y in x[a - 1..c].iter() {
            sum += y[b - 1..d].iter().fold(0, |sum, s| sum + s);
        }
        println!("{}", sum);
    }
}
