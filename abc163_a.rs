use proconio::input;

fn main() {
    input! {
        r:f64,
    }
    let pi = 2. * r * std::f64::consts::PI;
    print!("{}", pi);
}
