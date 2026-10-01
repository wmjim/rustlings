// TODO: 修复编译错误，但不要把宏定义移出这个模块。
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
