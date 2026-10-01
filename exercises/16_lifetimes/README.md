# 生命周期（Lifetime）

生命周期告诉编译器如何检查引用在任何具体情形下是否存活得足够久、从而保持有效。
例如生命周期表达的意思是：
“确保参数 'a' 至少活得和参数 'b' 一样久，这样返回值才是有效的”。

它们只在借用（也就是引用）上才需要，
因为被复制的参数或被移动（move）的值在它们所在的作用域中被拥有，
无法在作用域之外被引用。生命周期意味着函数的调用代码可以被检查，
以确保传入的实参是有效的。生命周期对其调用方是一种限制。

如果你想进一步了解生命周期标注，
[lifetimekata](https://tfpk.github.io/lifetimekata/) 项目
有一套与 Rustlings 风格相似的练习，专门用来学习如何编写生命周期标注。

## 延伸阅读

- [Lifetimes (in Rust By Example)](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
