use proconio::input;

fn main() {
    // Input
    input! {
        n: usize,
        cp: [(usize, usize);n],
        q: usize,
        lq: [(usize, usize);q],
    }

    // Solve
    let mut v1 = vec![0];
    v1.append(
        &mut cp
            .iter()
            .scan(0, |sum, val| {
                if val.0 == 1 {
                    *sum += val.1;
                }
                Some(*sum)
            })
            .collect::<Vec<usize>>(),
    );
    let mut v2 = vec![0];
    v2.append(
        &mut cp
            .iter()
            .scan(0, |sum, val| {
                if val.0 == 2 {
                    *sum += val.1;
                }
                Some(*sum)
            })
            .collect::<Vec<usize>>(),
    );

    // Output
    for (l, r) in lq {
        println!("{} {}", v1[r] - v1[l - 1], v2[r] - v2[l - 1])
    }
}
