use proconio::input;
use proconio::marker::Usize1;
use std::io::{stdout, BufWriter, Write};
use std::mem::swap;

struct UnionFind {
    par: Vec<usize>,
    siz: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            siz: vec![1; n],
        }
    }

    fn root(&self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.root(self.par[x])
    }

    fn is_same(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x_root = self.root(x);
        let mut y_root = self.root(y);

        if x_root == y_root {
            return false;
        }
        if self.siz[x_root] < self.siz[y_root] {
            swap(&mut x_root, &mut y_root);
        }
        self.par[y_root] = x_root;
        self.siz[x_root] += self.siz[y_root];
        true
    }

    fn size(&self, x: usize) -> usize {
        self.siz[self.root(x)]
    }

    fn is_connected(&self) -> bool {
        for i in 1..self.par.len() {
            if !self.is_same(0, i) {
                return false;
            }
        }
        true
    }
}

fn main() {
    input! {
        h: usize,w:usize,
        q: usize,
    }
    let d: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut writer = BufWriter::new(stdout());

    let mut m = vec![false; h * w];
    let mut uf = UnionFind::new(h * w);
    for _ in 0..q {
        input! {
            t: usize
        }
        if t == 1 {
            input! {
                y: Usize1, x: Usize1
            }
            let n1 = (y * w) + x;
            m[n1] = true;
            for dd in &d {
                let ny = y as isize + dd.0;
                let nx = x as isize + dd.1;
                if ny < 0 || ny >= h as isize || nx < 0 || nx >= w as isize {
                    continue;
                }
                let n2 = (ny as usize * w) + nx as usize;
                if m[n2] {
                    uf.unite(n1, n2);
                }
            }
        } else {
            input! {
                y1: Usize1, x1: Usize1,
                y2: Usize1, x2: Usize1
            }
            let n1 = (y1 * w) + x1;
            let n2 = (y2 * w) + x2;
            if uf.is_same(n1, n2) && m[n1] && m[n2] {
                writeln!(writer, "Yes").ok();
            } else {
                writeln!(writer, "No").ok();
            }
        }
    }
    writer.flush().unwrap();
}
