// `Vec<T>` 在类型 `T` 上是泛型的。大多数情况下，编译器都能推断出 `T`，
// 比如在向向量中 push 了一个具体类型的值之后。
// 但在这个练习里，编译器需要通过类型标注来获得一点帮助。

fn main() {
    // TODO: 通过给向量 `Vec<T>` 标注类型来修复编译错误。
    // 请选择一个既能从 `u8`、也能从 `i8` 创建出来的整数类型作为 `T`。
    let mut numbers = Vec::new();

    // 不要修改下面的代码。
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
