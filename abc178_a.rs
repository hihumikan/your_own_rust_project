use proconio::input;
fn main() {
    input! {
        x: i32,
    }
    if x == 1 {
        println!("{}", x - 1)
    } else {
        println!("{}", x + 1);
    }
}
