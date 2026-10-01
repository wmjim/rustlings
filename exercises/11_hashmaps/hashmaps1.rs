// 需要定义一个用哈希表表示的果篮。键（key）表示水果的名字，
// 值（value）表示篮子里这种水果有多少个。
// 你至少要往篮子里放 3 种不同的水果（比如 apple、banana、mango），
// 而且所有水果的总数至少要达到 5。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: 声明这个哈希表。
    // let mut basket =

    // 已经为你准备好两根香蕉了 :)
    basket.insert(String::from("banana"), 2);

    // TODO: 往篮子里多放一些水果。

    basket
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
