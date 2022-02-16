use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

const INF: usize = 18446744073709551615/3;

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
    // Input
    input! {
        n: usize,
    }
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            a: Usize1,
            b: Usize1
        }
        g[a].push(b);
        g[b].push(a);
    }

    // Solve
    let mut q = VecDeque::new();
    q.push_front(0);
    let mut dist = vec![INF; n];
    dist[0] = 0;
    while !q.is_empty() {
        let v = q.pop_back().unwrap();
        for &nv in &g[v] {
            if dist[nv] == INF {
                dist[nv] = dist[v] + 1;
                q.push_front(nv);
            }
        }
    }

    let mut idx = 0;
    let mut mdist = 0;
    for i in 0..n {
        if chmax!(mdist, dist[i]) {
            idx = i;
        }
    }

    let mut q = VecDeque::new();
    q.push_front(idx);
    let mut dist = vec![INF; n];
    dist[idx] = 0;
    let mut ans = 0;
    while !q.is_empty() {
        let v = q.pop_back().unwrap();
        for &nv in &g[v] {
            if dist[nv] == INF {
                dist[nv] = dist[v] + 1;
                q.push_front(nv);
                chmax!(ans, dist[nv]);
            }
        }
    }

    // Output
    println!("{}", ans + 1);
}
