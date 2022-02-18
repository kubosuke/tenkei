use proconio::input;
use proconio::marker::Chars;

const M: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let t = "atcoder".chars().collect::<Vec<char>>();

    // dp[i][j] = k
    // k: the number of ways that consist atcoder[:j] from s[:i]
    let mut dp = vec![vec![0; t.len() + 1]; n + 1];
    dp[0][0] = 1;

    // (i,j) => (i+1, j)
    //       => (i+1, j+1)
    for i in 0..n {
        for j in 0..=t.len() {
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= M;
            if j < t.len() {
                if s[i] == t[j] {
                    dp[i + 1][j + 1] += dp[i][j];
                    dp[i + 1][j + 1] %= M;
                }
            }
        }
    }
    println!("{}", dp[n][t.len()]);
}
