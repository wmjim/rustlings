fn factorial(num: u64) -> u64 {
    // TODO: 补全这个函数，让它返回 `num` 的阶乘，
    // 阶乘的定义是 `1 * 2 * 3 * … * num`。
    // https://en.wikipedia.org/wiki/Factorial
    //
    // 不要使用：
    // - 提前返回（显式使用 `return` 关键字）
    // 尽量不要使用：
    // - 命令式风格的循环（for/while）
    // - 额外的变量
    // 想向更高难度挑战的话，不要使用：
    // - 递归
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
