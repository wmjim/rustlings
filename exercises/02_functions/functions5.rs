// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i32 {
    // 代码末尾使用表达式以便隐式返回结果，也可在其前添加 return 显式返回
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
