use proconio::input;
use std::f64::consts::PI;

fn main() {
    input! {
        t: f64,
        l: f64, x: f64, y: f64,
        q: usize,
        e: [f64; q],
    }
    for ee in e {
        let theta = ee * 2.0 * PI / t;
        let height = -(l / 2.0) * theta.cos() + l / 2.0;
        let yy = -(l / 2.0) * theta.sin();
        let dist = (x * x + (y - yy) * (y - yy)).sqrt();
        println!("{}", 90.0 - (dist / height).atan().to_degrees());
    }
}
