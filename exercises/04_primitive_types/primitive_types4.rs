fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: 从数组 `a` 中切出一个名为 `nice_slice` 的 slice，让测试通过。
        // let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice);
    }
}
