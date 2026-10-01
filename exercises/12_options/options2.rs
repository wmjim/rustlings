fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: 把它改成取值形式为 `Some` 的 if let 语句。
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: 把它改成 while let 语句。记住 `Vec::pop()` 会再加上一层 `Option`。
        // 在 if let 和 while let 语句里可以做嵌套的模式匹配。
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
