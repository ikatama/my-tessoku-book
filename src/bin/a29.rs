use num_traits::PrimInt;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    let p = 10.pow(9) + 7;
    let result = modpow(a, b, p);
    println!("{}", result);
}

fn modpow(mut a: u64, mut b: u64, p: u64) -> u64 {
    let mut result = 1;
    while b > 0 {
        if b & 1 > 0 {
            result = result * a % p
        }
        a = a * a % p;
        b >>= 1;
    }
    result
}
