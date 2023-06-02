use proconio::input;

fn main() {
    input! {
        n: i32,
        x: i32,
        a: [i32;n],
    }

    let result: &str = if a.contains(&x) { "Yes" } else { "No" };
    println!("{}", result);
}
