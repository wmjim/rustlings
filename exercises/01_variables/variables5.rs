fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    // TODO: Fix the compiler error by changing the line below without renaming the variable.
    // 使用变量遮蔽，重新声明变量 x 的绑定
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
