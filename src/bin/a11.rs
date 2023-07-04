use proconio::input;

fn main() {
    // 2分探索で解く
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let answer = a.binary_search(&x);
    println!("{}", answer.unwrap() + 1);
}
