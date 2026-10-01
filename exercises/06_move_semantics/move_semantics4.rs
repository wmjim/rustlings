fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    // TODO: 只通过调整测试中代码行的顺序来修复编译错误。
    // 不要新增、修改或删除任何一行。
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        let z = &mut x;
        y.push(42);
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
