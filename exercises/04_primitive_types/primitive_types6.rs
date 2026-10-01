fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: 用元组索引访问 `numbers` 的第二个元素，
        // 并把它赋值给名为 `second` 的变量。
        // let second = ???;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
