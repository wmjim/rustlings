# Option

`Option` 类型表示一个可选的值：每个 `Option` 要么是 `Some` 并且包含一个值，
要么是 `None` 并且不包含值。
`Option` 在 Rust 代码中非常常见，它有很多用途：

- 初始值
- 未在整个输入范围上都有定义的函数（部分函数）的返回值
- 用于报告简单错误的返回值，出错时返回 `None`
- 可选的结构体字段
- 可以被借用或“取走”的结构体字段
- 可选的函数参数
- 可空的指针
- 从棘手情形中换出值

## 延伸阅读

- [Option Enum Format](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
