trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: 为字符串向量实现 `AppendBar` trait。
// `append_bar` 应该把字符串 "Bar" push 到向量里。

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
