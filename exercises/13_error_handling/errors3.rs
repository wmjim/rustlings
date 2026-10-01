// 这个程序想使用上一个练习中已完成的 `total_cost` 函数。
// 但它跑不起来！为什么呢？我们该怎么做才能修好它？

use std::num::ParseIntError;

// 不要修改这个函数。
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: 通过修改 `main` 函数的签名和函数体来修复编译错误。
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // 不要修改这一行。
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
}
