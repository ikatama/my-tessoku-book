use proconio::input;

fn main() {
    input! {
        n: i32,
        k: i32,
        p: [i32;n],
        q: [i32;n]
    }

    for a in p {
        let b = k - a;
        if q.contains(&b) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
