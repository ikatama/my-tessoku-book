use proconio::input;

fn main() {
    input! {
        q: usize,
        x: [u32; q],
    }

    for i in x {
        if is_prime(i) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn is_prime(n: u32) -> bool {
    let mut i = 2;

    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    return true;
}
