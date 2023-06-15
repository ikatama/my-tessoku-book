use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }

    let mut x: Vec<usize> = Vec::new();
    let mut y: Vec<usize> = Vec::new();

    // 配列の数を減らす
    for i in 0..n {
        for j in 0..n {
            x.push(a[i] + b[j]);
            y.push(c[i] + d[j]);
        }
    }

    y.sort();

    for e in x {
        if k <= e {
            continue;
        }

        let mut left = 0;
        let mut right = n.pow(2);

        let diff = k - e;
        while left < right {
            let mid = (left + right) / 2;
            if diff == y[mid] {
                println!("Yes");
                return;
            } else if diff < y[mid] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
    }
    println!("{}", "No")
}
