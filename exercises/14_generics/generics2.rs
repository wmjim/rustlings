// 这个强大的包装类型可以用来存储一个正整数。
// TODO: 用泛型重写它，让它能包装任意类型。
struct Wrapper {
    value: u32,
}

// TODO: 修改这个结构体的实现，让它在被包装的值上具有泛型。
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
