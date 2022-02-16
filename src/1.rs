use proconio::input;

fn main() {
    // Input
    input! {
        n: usize, l: usize,
        k: usize,
        a: [usize; n]
    }

    // Solve
    let mut left = 0;
    let mut right = l;
    while right - left > 1 {
        let mid = (right + left) / 2;

        let mut pre = 0;
        let mut cnt = 0;
        for i in 0..n {
            let now = a[i] - pre;
            if now >= mid {
                pre = a[i];
                cnt += 1;
            }
        }
        if l - pre >= mid {
            cnt += 1;
        }
        if cnt < k + 1 {
            right = mid;
        } else {
            left = mid;
        }
    }

    // Output
    println!("{}", left);
}
