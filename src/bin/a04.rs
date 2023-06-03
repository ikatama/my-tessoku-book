use proconio::input;

fn main() {
    input! {
        n: i32,
    }
    let converted = convert_radix(n, 2);
    let result: String = converted
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .concat();

    println!("{:0>10}", result);
}

fn convert_radix(mut n: i32, b: i32) -> Vec<i32> {
    let mut remainder: Vec<i32> = Vec::new();

    while n != 0 {
        remainder.push(n % b);
        n = n / b;
    }

    remainder.reverse();
    return remainder;
}
