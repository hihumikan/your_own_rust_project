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

    //警告文
    assert!(0 == x); //条件が成り立っている場合は警告は発生しない。
    assert!(0 == x, "メッセージ");
    assert!(0 == x, "{}", x);
    //or
    assert_eq!(0, x);
    assert_eq!(0, x);

    assert!(0 != x);
    //or
    assert_ne!(0, x);

    //複数変数宣言
    let test: (i32, f64) = (10, 20_f64);
    let test2: test;
    let (x, y) = test;
    assert_eq!(x, 10);
    println!("{}", test.0);
    println!("{}", x);

    //型で宣言する時にif文が書ける
    let (max, min) = if a > b { (a, b) } else { (b, a) };


    let test:i32;
    test = {
        println!("Hello");
        10
    };
    assert_eq!(test,10);

    //要素0
    let unit: ();
    unit = {
        println!("() を返すブロック");
    };
    assert_eq!(unit, ());

    //配列
    let array: [i64; 3];
    array = [0,1,2];
    //or
    let array = [1, 2, 3];
    let array = [57;10];

    println!("{1} {0} {1}",1,2 );
    // 2 1 2

}
