#![feature(proc_macro_hygiene, decl_macro)]

//Rocketで多用されるマクロのための設定
#[macro_use]
extern crate rocket;

//指定されたパス以下で
//"/"にGETリクエストが来た場合は、この関数が動作する
#[get("/")]
fn index() -> &'static str {
    let hello = "Hello, world!\n\nこんにちは\n\nSep/16/2020\n";

    hello
}

#[get("/about")]
fn about() -> &'static str {
    let about = "aboutの話です。";

    about
}

fn main() {
    rocket::ignite().mount("/", routes![index, about]).launch();
}
