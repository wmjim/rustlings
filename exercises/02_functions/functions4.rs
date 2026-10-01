// 这家商店正在促销：如果价格是偶数，可以减免 10 Rustbuck；
// 如果是奇数，则减免 3 Rustbuck。
// 暂时不用关心函数体本身，我们现在只关注函数签名。

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: 修复函数签名。
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
