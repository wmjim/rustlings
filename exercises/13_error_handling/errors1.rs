// TODO: 如果传入空字符串，这个函数就拒绝生成要打印在名牌上的文字。
// 如果它能说明问题出在哪里，而不是只返回 `None`，那就更好了。
// 幸运的是，Rust 有一个与 `Option` 类似、可以用来表达错误状况的构造。
// 请修改函数签名和函数体，让它返回 `Result<String, String>`
// 而不是 `Option<String>`。
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // 不允许空名字
        None
    } else {
        Some(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
