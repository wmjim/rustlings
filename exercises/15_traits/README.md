# Trait

trait 是一组方法的集合。

数据类型可以实现 trait。要做到这一点，就要为该数据类型定义组成这个 trait 的那些方法。
例如，`String` 数据类型实现了 `From<&str>` trait，因此我们可以写 `String::from("hello")`。

从这个角度看，trait 有点像 Java 的接口和 C++ 的抽象类。

Rust 中其他一些常见的 trait 还有：

- `Clone`（提供 `clone` 方法）
- `Display`（支持用 `{}` 做格式化输出）
- `Debug`（支持用 `{:?}` 做格式化输出）

由于 trait 表示数据类型之间的共同行为，所以在编写泛型时它们很有用。

## 延伸阅读

- [Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
