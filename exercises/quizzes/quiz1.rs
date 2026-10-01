// 这个测验考查以下小节：
// - 变量
// - 函数
// - If
//
// 玛丽在买苹果。苹果的价格这样计算：
// - 一个苹果 2 rustbuck。
// - 但是，如果玛丽买的苹果超过 40 个，
//   整笔订单中每个苹果的价格就降到只要 1 rustbuck！

// TODO: 写一个函数，根据购买数量计算一笔苹果订单的价格。
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn main() {
    // 你可以在这里随意试验。
}

// 不要修改测试！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
