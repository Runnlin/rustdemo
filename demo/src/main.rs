
#![allow(unused)]
fn main() {
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // 强大之处在于：match可以返回任何值！
    // => 后面是会被运行的代码
    match coin {
        Coin::Penny => {
            println!("Luckly penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
        // Default
        _ => 0,
    }

}

if let 3 = 3 {
    println!("Three is three!");
}
}
