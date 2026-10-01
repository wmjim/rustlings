// AsRef 和 AsMut 可以进行开销很小的引用到引用的转换。
// 关于它们，可以分别阅读
// https://doc.rust-lang.org/std/convert/trait.AsRef.html 和
// https://doc.rust-lang.org/std/convert/trait.AsMut.html。

// 获取给定参数中的字节数（不是字符数）。
// （`.len()` 返回字符串中的字节数。）
// TODO: 恰当地把 `AsRef` trait 作为 trait bound 加上。
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

// 获取给定参数中的字符数（不是字节数）。
// TODO: 恰当地把 `AsRef` trait 作为 trait bound 加上。
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 用 `as_mut()` 求一个数的平方。
// TODO: 加上合适的 trait bound。
fn num_sq<T>(arg: &mut T) {
    // TODO: 实现函数体。
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
