fn main() {
    //改行有
    println!("hello");
    eprintln!("error");
    //改行無
    print!("hello");
    eprint!("error");
    //実行時エラー
    //panic!();
    //四則演算
    println!("{},{}", 6 + 3 - 1 / 2, 31 % 2);

    //変数
    //a~z,A~Z,0~9,_
    let hoge = 20_i64;
    let fhoge = 112.313_f64;
    println!("{}", hoge);
    println!("{}", fhoge);
    //https://doc.rust-lang.org/std/index.html#primitives
    let hoge2: i32 = 20;

    //入力受取
    //https://crates.io/crates/proconio
    proconio::input! {
        n: i32,
        y: i32,
    }
    //or
    use proconio::input;

    //if文
    if n == 0 && y == 0 {
        println!("");
    } else if !(n > 0) {
        println!("");
    } else {
        println!("");
    }
    //or
    let hoge3 = if n == 0 { y + 100 } else { y + 200 };
}
