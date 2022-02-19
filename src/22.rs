use proconio::input;

pub fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn main() {
    input! {
        a: u64, b: u64, c: u64
    }
    let g = gcd(a, gcd(b, c));
    println!("{}", a / g + b / g + c / g - 3);
}
