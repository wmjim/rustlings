fn main() {
    my_macro!();
}

// TODO: 把这个宏的整个定义挪个位置，修复编译错误。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
