# 类型转换

Rust 提供了许多把某种类型的值转换成另一种类型的途径。

最简单的类型转换是类型转换表达式，用二元运算符 `as` 表示。
例如 `println!("{}", 1 + 1.0);` 无法通过编译，因为 `1` 是整数而 `1.0` 是浮点数；
但 `println!("{}", 1 as f32 + 1.0)` 可以编译通过。
[`using_as`](using_as.rs) 这个练习讲的就是这个。

Rust 还提供了一些 trait，只要实现它们就能进行类型转换。
这些 trait 都位于 [`convert`](https://doc.rust-lang.org/std/convert/index.html) 模块中。
它们包括：

- `From` 和 `Into`，见 [`from_into`](from_into.rs)
- `TryFrom` 和 `TryInto`，见 [`try_from_into`](try_from_into.rs)
- `AsRef` 和 `AsMut`，见 [`as_ref_mut`](as_ref_mut.rs)

此外，`std::str` 模块还提供了一个叫 [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) 的 trait，
它借助字符串上的 `parse` 方法把字符串转换成目标类型。
如果为某个类型 `Person` 正确实现了它，
那么 `let p: Person = "Mark,20".parse().unwrap()` 既能编译通过，运行时也不会 panic。

这些应该就是***在标准库范围内***把数据转换成你想要的类型的主要途径。

## 延伸阅读

书中并没有直接讲到这些内容，不过标准库为此提供了很棒的文档。

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)
