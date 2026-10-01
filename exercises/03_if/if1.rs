fn bigger(a: i32, b: i32) -> i32 {
    // TODO: 补全这个函数，让它返回较大的那个数！
    // 如果两个数相等，返回其中任意一个都可以。
    // 不要使用：
    // - 调用其他函数
    // - 额外的变量
}

fn main() {
    // 你可以在这里随意试验。
}

// 暂时不用管这个 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
