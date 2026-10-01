// 测试很重要，它能确保你的代码确实按照你以为的方式工作。

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    // TODO: 导入 `is_even`。你可以用通配符把外层模块里的东西全部导入。

    #[test]
    fn you_can_assert() {
        // TODO: 用一些值来测试 `is_even` 函数。
        assert!();
        assert!();
    }
}
