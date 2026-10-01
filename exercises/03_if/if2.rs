// TODO: 修复这个函数中的编译错误。
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else {
        1
    }
}

fn main() {
    // 你可以在这里随意试验。
}

// TODO: 阅读测试，理解期望的行为。
// 在不修改测试的前提下让所有测试通过。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // 意思是：用参数 "strawberry" 调用 `picky_eater` 时，应该返回 "Yummy!"。
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
