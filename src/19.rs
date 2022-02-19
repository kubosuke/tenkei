use proconio::input;

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

fn main() {
    input! {
        mut n: usize,
        a: [i32; 2*n],
    }
    // the minimum cost of [l,r]
    let mut dp = vec![vec![0; n * 2]; n * 2];
    for w in (1..n * 2).step_by(2) {
        for l in 0..n * 2 - w {
            let r = l + w;
            dp[l][r] = 1_000_000_000;
            chmin!(dp[l][r], dp[l + 1][r - 1] + (a[l] - a[r]).abs());
            for i in (l + 1..r).step_by(2) {
                chmin!(dp[l][r], dp[l][i] + dp[i + 1][r]);
            }
        }
    }
    println!("{}", dp[0][n * 2 - 1]);
}
