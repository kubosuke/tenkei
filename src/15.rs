use proconio::input;

const MOD: u64 = 1_000_000_007;

fn inv(mut a: u64) -> u64 {
    let mut t = 1;
    let mut n = MOD - 2;
    while n > 0 {
        if n & 1 == 1 {
            t = t * a % MOD;
        }
        a = a * a % MOD;
        n >>= 1;
    }
    t
}

fn main() {
    input!(n: usize);
    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = i as u64 * fact[i - 1] % MOD;
    }
    let mut ifact = vec![1; n + 1];
    ifact[n] = inv(fact[n]);
    for i in (1..n).rev() {
        ifact[i] = (i + 1) as u64 * ifact[i + 1] % MOD;
    }
    let combination = |n: usize, r: usize| -> u64 { fact[n] * ifact[r] % MOD * ifact[n - r] % MOD };
    for k in 1..=n {
        let mut ans = 0;
        for i in 1.. {
            let nu = n as isize - (i - 1) * (k - 1) as isize;
            if nu < i {
                break;
            }
            ans += combination(nu as usize, i as usize);
        }
        ans %= MOD;
        println!("{}", ans);
    }
}
