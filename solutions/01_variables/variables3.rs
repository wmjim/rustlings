#![allow(clippy::needless_late_init)]

fn main() {
    // Rust 中不允许读取未初始化的变量！
    // 因此我们需要先给它赋一个值。
    let x: i32 = 42;

    println!("Number {x}");

    // 也可以先声明变量，之后再初始化。
    // 但在初始化之前不能使用它。
    let y: i32;
    y = 42;
    println!("Number {y}");
}
