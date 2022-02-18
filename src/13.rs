use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const INF: usize = 18446744073755;

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
        n: usize, m: usize,
    }

    let mut g = vec![vec![]; n];
    for _ in 0..m {
        input! {
            s: Usize1, t: Usize1, c: usize
        }
        g[s].push((t, c));
        g[t].push((s, c));
    }

    // Dijkstra
    // 0 to each nodes
    let mut dist1 = vec![INF; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    dist1[0] = 0;
    while !heap.is_empty() {
        let (Reverse(d), v) = heap.pop().unwrap();
        if dist1[v] != d {
            continue;
        };
        for &(to, cost) in &g[v] {
            if chmin!(dist1[to], dist1[v] + cost) {
                heap.push((Reverse(dist1[to]), to));
            }
        }
    }

    // Dijkstra
    // n to each nodes
    let mut dist2 = vec![INF; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), n - 1));
    dist2[n - 1] = 0;
    while !heap.is_empty() {
        let (Reverse(d), v) = heap.pop().unwrap();
        if dist2[v] != d {
            continue;
        };
        for &(to, cost) in &g[v] {
            if chmin!(dist2[to], dist2[v] + cost) {
                heap.push((Reverse(dist2[to]), to));
            }
        }
    }

    // Output
    for i in 0..n {
        println!("{}", dist1[i] + dist2[i]);
    }
}
