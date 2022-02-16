use proconio::input;

fn main() {
    // Input
    input! {
        n: usize
    }

    // Solve
    if n % 2 == 1 {
        return;
    }
    for i in 0..2u32.pow(n as u32) {
        let b = format!("{:0width$b}", i, width = n);
        let mut cnt = 0;
        let mut f = true;
        for c in b.chars() {
            if cnt < 0 {
                f = false
            }
            if c == '0' {
                cnt += 1
            } else {
                cnt -= 1
            }
        }
        if f && cnt == 0 {
            println!("{}", b.replace("0", "(").replace("1", ")"))
        }
    }
}
