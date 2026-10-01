fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: 把 `input` slice 中的每个元素乘以 2，然后 push 到 `output` 向量里。
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // 一个先用 map 映射、再用 collect 收集成向量的例子。
    // 我们把 `input` slice 中的每个元素映射为它的值加 1。
    // 如果输入是 `[1, 2, 3]`，输出就是 `[2, 3, 4]`。
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: 这里我们同样要把 `input` slice 中的每个元素乘以 2，
    // 但要用迭代器的 map，而不是手动 push 到一个空向量里。
    // 参考上面 `vec_map_example` 函数中的例子。
    input
        .iter()
        .map(|element| {
            // ???
        })
        .collect()
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
