use proconio::input;

fn main() {
    input! {
        a: u128, b: u32, c: u128
    }
    println!("{}", {
        if a < c.pow(b) {
            "Yes"
        } else {
            "No"
        }
    })
}
