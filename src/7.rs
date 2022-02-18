use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        q: usize,
        b: [isize; q]
    }
    a.sort();

    for bb in b {
        let mut left = 0;
        let mut right = n;
        while right - left > 1 {
            let mid = (right + left) / 2;
            if a[mid as usize] > bb {
                right = mid;
            } else {
                left = mid;
            }
        }
        if right < n {
            println!(
                "{}",
                std::cmp::min((bb - a[left]).abs(), (a[right] - bb).abs())
            );
        } else {
            println!("{}", (bb - a[left]).abs());
        };
    }
}
