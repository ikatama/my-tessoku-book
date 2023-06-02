use proconio::input;

fn main() {
    input! {
        n: u32
    };

    let result: u32 = n * n;
    println!("{}", result.to_string());
}
