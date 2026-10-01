# 向量（Vector）

向量是 Rust 中最常用的数据结构之一。在其他编程语言里，它们可能就直接叫数组，
但由于 Rust 的抽象层次更低一些，Rust 中的数组（array）存放在栈上（也就是说它
既不能增长也不能缩小，长度必须在编译期已知），而向量（Vector）存放在堆上
（因此没有这些限制）。

向量在书中属于比较靠后的章节，但我们觉得它足够有用，值得提前一点介绍。
另一个同样有用的数据结构——哈希表（hash map）——我们稍后再讲。

## 延伸阅读

- [Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
