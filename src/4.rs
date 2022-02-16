use proconio::input;
const INF: usize = 18446744073709551615;

fn main() {
    // Input
    input! {
        h: usize, w: usize,
        a: [[usize;w]; h]
    }

    // Solve
    let mut memo_y = vec![INF; h];
    let mut memo_x = vec![INF; w];
    for y in 0..h {
        if memo_y[y] == INF {
            memo_y[y] = a[y].iter().sum::<usize>();
        }
        let ho = memo_y[y];
        for i in 0..w {
            if memo_x[i] == INF {
                memo_x[i] = {
                    let mut tmp = 0;
                    for j in 0..h {
                        tmp += a[j][i]
                    }
                    tmp
                };
            }
            let ve = memo_x[i];
            print!("{} ", ho + ve - a[y][i]);
        }
        println!()
    }
}
