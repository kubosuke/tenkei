use proconio::input;
use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String,
    }
    let mut heap = BinaryHeap::new();
    let mut next = 0;
    let mut ans = String::new();
    for (i, c) in s.chars().enumerate() {
        heap.push(Reverse((c, i)));
        if n <= i + k {
            loop {
                let Reverse((d, j)) = heap.pop().unwrap();
                if next <= j {
                    next = j + 1;
                    ans.push(d);
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}
