use ordered_float::OrderedFloat;
use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        n: usize,
        xy: [(f64, f64); n],
    }
    let mut result = 0.;
    let mut angles = vec![];

    // Fix a point I
    for i in 0..n {
        let (xi, yi) = xy[i];
        angles.clear();
        // 偏角ソート
        for j in 0..n {
            if i != j {
                let (xj, yj) = xy[j];
                // point I を原点に移動させたときの point J の偏角θを求める
                let t = (yj - yi).atan2(xj - xi);

                // 円環2倍テク: https://raw.githubusercontent.com/E869120/kyopro_educational_90/main/editorial/076.jpg
                // -Pi <= θ <= Pi の範囲
                // 2倍して -2Pi <= θ <= 2Pi
                // 右にずらして 0 <= θ <= 4Pi
                if t >= 0. {
                    angles.push(t);
                    angles.push(t + 2. * PI);
                } else {
                    angles.push(t + 2. * PI);
                    angles.push(t + 4. * PI);
                }
            }
        }
        angles.sort_by_key(|&v| OrderedFloat(v));

        // 尺取り法
        let mut k = 0;
        // 左腕jをずらす
        for j in 0..n - 1 {
            // PIを超えないように右腕kをずらす
            while angles[k + 1] - angles[j] <= PI {
                k += 1;
            }
            // ギリ180を超えないtが候補となる
            let t = angles[k] - angles[j];
            if t > result {
                result = t;
            }
        }
    }
    println!("{}", result.to_degrees());
}
