use proconio::input;

const M: usize = 1000000007;

fn main() {
    // Input
    input! {
        n: usize, b: usize, k: usize,
        c: [usize; k]
    }

    // Solve
    // dp[i][r]
    // the number of i-digit integer whose modulo of B is r
    let mut dp = vec![vec![0; b]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..b {
            for cc in &c {
                dp[i + 1][(10 * j + cc) % b] += dp[i][j];
                dp[i + 1][(10 * j + cc) % b] %= M;
            }
        }
    }

    println!("{}", dp[n][0]);
}
