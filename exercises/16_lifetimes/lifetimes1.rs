// Rust 编译器需要知道如何检查传入的引用是否有效，
// 这样它才能在某个引用有“在被使用之前就离开作用域”的风险时提醒程序员。
// 记住，引用是借用，它们并不拥有自己的数据。
// 如果数据的所有者离开了作用域会怎样？

// TODO: 通过修改函数签名来修复编译错误。
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
