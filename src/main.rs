use proconio::input;

fn main() {
    let array = [0, 10, 20, 30, 40, 50];
    input! {
        index: usize,
    }
    let ans = array[index];
    println!("{:^6}", ans);
}