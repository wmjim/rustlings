// 在编译期，Rust 需要知道一个类型占多少空间。对递归类型来说这会有问题：
// 因为一个值可能把另一个同类型的值作为自己的一部分。
// 为了解决这个问题，我们可以使用 `Box` —— 一种把数据存放在堆上的智能指针，
// 它也让我们可以包装递归类型。
//
// 这个练习中我们要实现的递归类型是 "cons list"，
// 它是函数式编程语言中常见的一种数据结构。
// cons list 中的每一项包含两部分：当前项的值，以及下一项。
// 最后一项是一个叫做 `Nil` 的值。

// TODO: 在枚举定义中使用 `Box`，让代码能够编译。
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO: 创建一个空的 cons list。
fn create_empty_list() -> List {
    todo!()
}

// TODO: 创建一个非空的 cons list。
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
