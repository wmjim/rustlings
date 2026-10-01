fn main() {
    // 在 Rust 中，变量默认是不可变的。
    // 在 `let` 后面加上 `mut` 关键字，就能让声明的变量变为可变。
    let mut x = 3;
    println!("Number {x}");

    x = 5;
    println!("Number {x}");
}
