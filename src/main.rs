mod sena;

use sena::hoge;//他の.rsにある関数・変数を使う際の書き方

fn main() {
    println!("Hello, world!");
    // let start = 114514u64;
    // let mut value = start;
    // loop {
    //     value = f(value);
    //     println!("{}", value);
    //     if value == 1 {
    //         break;
    //     }
    // }

    let success = hoge(13, |v| v - 1);
    println!("success?: {}", success);
}

fn f(n: u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

// Matrixはコピーを備えていないので&で参照する必要あり(もしくはクローン)
// mutは可変だが、そのままletは値を変えられない
// lib.rsもしくは main.rsがルートファイルと決まっている、見たいファイルは自分で指定しなければならない