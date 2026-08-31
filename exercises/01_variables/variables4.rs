// TODO: Fix the compiler error.
fn main() {
    // 变量默认是不可变的，但可在 let 后添加 mut 关键字声明变量可变
    let mut x = 3;
    println!("Number {x}");

    x = 5; // Don't change this line
    println!("Number {x}");
}
