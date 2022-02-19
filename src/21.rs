use petgraph::algo::tarjan_scc;
use petgraph::graph::Graph;
use proconio::input;

fn main() {
    input! {
        _: usize, m: usize,
        uv: [(u32, u32); m]
    }
    let g = Graph::<u32, u32>::from_edges(&uv);
    let scc = tarjan_scc(&g);

    println!(
        "{}",
        scc.iter()
            .fold(0, |sum, v| sum + v.len() * (v.len() - 1) / 2)
    );
}
