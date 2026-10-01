// Clippy 是一个 lint 工具集，它负责分析你的代码，
// 帮你发现常见错误并改进 Rust 代码。
//
// 在这些练习中，只要存在 Clippy 警告，代码就无法通过编译。
// 请查看输出中 Clippy 给出的建议来解决练习。

fn main() {
    // TODO: 修复这一行上的 Clippy lint。
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
