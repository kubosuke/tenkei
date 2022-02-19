macro_rules! chmin {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_min = min!($($cmps),+);
        if $base > cmp_min {
            $base = cmp_min;
            true
        } else {
            false
        }
    }};
}

macro_rules! min {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::min($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::min($a, min!($($rest),+))
    }};
}

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize, b: usize, c: usize
    }
    let mut ans = 10000;

    for i in 0..=10000 {
        let mut j = 0;
        while (i + j) < 10000 {
            let x = a * i + b * j;
            if n < x {
                j += 1;
                break;
            }
            if (n - x) % c == 0 {
                chmin!(ans, i + j + ((n - x) / c));
            }
            j += 1;
        }
    }
    println!("{}", ans)
}
