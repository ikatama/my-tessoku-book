use proconio::input;

fn main() {
    input! {
        a: u32,
        b: u32,
    }

    // ユークリッドの互除法
    println!("{}", gcd(a, b))
}

fn gcd(a: u32, b: u32) -> u32 {
    if a == 0 {
        return b;
    } else {
        gcd(b % a, a)
    }
}
