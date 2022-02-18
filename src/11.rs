use proconio::input;

macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

fn main() {
    input! {
        n: usize,
        mut dcs: [(usize, usize, usize); n],
    }
    dcs.sort_by_key(|(d, _, _)| *d);

    // dp[i][j]: 制約「i個目までのお仕事、j日経過した状態」の最大
    let mut dp = vec![vec![0; 5010]; n + 1];

    for i in 0..n {
        for j in 0..5010 {
            // pass i-th task
            chmax!(dp[i + 1][j], dp[i][j]);

            // do i-th task
            if dcs[i].1 + j <= dcs[i].0 {
                chmax!(dp[i + 1][j + dcs[i].1], dp[i][j] + dcs[i].2);
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}
