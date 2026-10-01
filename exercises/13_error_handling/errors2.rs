// 假设我们在写一个游戏：玩家可以用代币购买物品。每件物品 5 个代币，
// 每次购买还要额外支付 1 个代币的手续费。玩家会输入想买多少件物品，
// `total_cost` 函数负责算出这些物品的总花费。由于数量是玩家输入的，
// 我们拿到的是一个字符串。他可能输入任何东西，而不只是数字！
//
// 目前这个函数完全没有处理出错的情况。我们想要的行为是：
// 如果我们用一个不是数字的字符串调用 `total_cost`，
// 它应该返回一个 `ParseIntError`。这种情况下，我们希望立即把这个错误
// 从函数里返回出去，而不要继续尝试做乘法和加法。
//
// 至少有两种正确的实现方式，但其中一种要短得多！

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: 按上面描述的方式处理出错的情况。
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
