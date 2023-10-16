use proconio::input;

fn main() {
    input! {
        n: i32,
        t_a: [(char, i32); n],
    }

    let mut result = 0;
    for t_a_iter in t_a.into_iter() {
        match t_a_iter.0 {
            '+' => result += t_a_iter.1,
            '-' => result -= t_a_iter.1,
            '*' => result *= t_a_iter.1,
            _ => panic!(),
        }
        result = if result < 0 { result + 10000 } else { result };
        result %= 10000;
        println!("{}", result);
    }
}
