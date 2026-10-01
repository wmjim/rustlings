# Rustlings 提示中文对照（HINTS.zh-CN.md）

在终端里输入 `h`（或运行 `rustlings hint <练习名>`）时，Rustlings 显示的提示文字是**编译在 rustlings 程序内部**的，不在本仓库的文件里，因此无法通过修改仓库文件来改变它的语言。本文件是把这些内置提示翻译成中文后的对照表，方便你在看英文提示时随时查阅。

用法：在下面找到你正在做的练习名，对照阅读即可。

- 练习题在 `exercises/`，参考答案在 `solutions/`（做完练习后由 rustlings 自动填入）。
- `rustlings hint` 不带参数时，显示下一个待完成练习的提示。

---

## 00_intro

### intro1

输入 `n` 进入下一个练习。
输入 `n` 之后可能需要再按一次 ENTER。

### intro2

编译器在告诉我们，打印宏的名字写错了，并且给出了一个替代写法。

## 01_variables

### variables1

`main` 函数中的这个声明缺少一个关键字，在 Rust 中创建新的变量绑定需要它。

### variables2

编译器的意思是：根据这里给出的信息，Rust 无法推断变量绑定 `x` 的类型。

- 如果给 `main` 函数的第一行加上类型标注，会怎么样？
- 如果给 `x` 一个值，会怎么样？
- 如果两件事都做呢？
- 那么 `x` 到底应该是什么类型？
- 如果 `x` 和 `10` 是同一个类型呢？如果类型不同呢？

### variables3

这个练习里，我们在 `main` 函数中创建了一个变量绑定，并想在下一行使用它，
但我们没有给它任何值。

我们无法打印一个根本不存在的东西；试着给 `x` 一个值吧！

这个错误在任何编程语言里都很容易造成 bug —— 好心的 Rust 编译器帮我们抓住了它！

### variables4

在 Rust 中，变量绑定默认是不可变的。但这里我们试图给 `x` 赋一个新的值！
有一个关键字可以让变量绑定变成可变的。

### variables5

在 `variables4` 中我们已经学会了用某个特殊关键字把不可变变量变成可变变量。
可惜在这个练习里它帮不上太多忙，因为我们想给一个已存在的变量赋上不同类型
的值。有时你也会想复用已经存在的变量名，因为你只是像这个练习一样
把值转换成别的类型。

幸运的是，Rust 对这个问题有一个强大的解决方案：“遮蔽”（Shadowing）！
你可以在书中“Variables and Mutability”一节里读到更多关于遮蔽的内容：
https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing

之后试着用这个技巧解决这个练习。

### variables6

我们已经了解了变量和可变性，但还有一种重要的“变量”类型：常量（constant）。

常量永远是不可变的，它们用关键字 `const` 声明，而不是 `let`。

常量的类型必须始终标注出来。

在书中“Variables and Mutability”一节的 “Constants” 部分，
可以读到更多关于常量、以及变量与常量区别的内容：
https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants

## 02_functions

### functions1

这个 `main` 函数调用了一个它认为存在的函数，但这个函数并不存在。
它期望这个函数叫 `call_me`，不接收参数，也不返回值。
听起来很像 `main`，对吧？

### functions2

Rust 要求函数签名的每一部分都要有类型标注，但 `call_me` 缺少了 `num` 的类型标注。

### functions3

这一次，函数的*声明*没问题，但调用它的地方有点问题。

### functions4

错误信息指向了函数 `sale_price`，说它在 `->` 之后缺少一个类型。
这里应该写函数的返回类型。
看看 `is_even` 函数的例子吧！

### functions5

这是一个非常常见的错误，只要删掉一个字符就能修好。
它出现的原因是 Rust 区分表达式（expression）和语句（statement）：
表达式会基于操作数返回一个值，而语句只是返回 `()` 类型，
它的行为就像 C/C++ 里的 `void`。

我们想让 `square` 函数返回 `i32` 类型的值，但它返回的是 `()` 类型。

有两种解决方案：
1. 在 `num * num;` 前面加上 `return` 关键字
2. 去掉 `num * num` 后面的分号 `;`

## 03_if

### if1

如果你愿意，完全可以只用一行做到！

其他语言里类似写法的例子：
- C(++) 里是：`a > b ? a : b`
- Python 里是：`a if a > b else b`

请记住在 Rust 中：
- `if` 条件不需要用圆括号包起来
- `if`/`else` 条件分支是表达式
- 每个条件后面跟一个 `{}` 代码块

### if2

关于第一个编译错误：在 Rust 中很重要的一点是，每个条件分支块都要返回相同的类型！

要让测试通过，你需要几个检查不同输入值的条件。
读一读测试，弄清它们期望什么。

### if3

在 Rust 中，`if` 表达式的每个分支都必须返回相同类型的值。
确保所有分支的类型保持一致。

## quizzes

### quiz1

这次没有提示 ;)

## 04_primitive_types

### primitive_types1

在 Rust 中，布尔值可以用运算符 `!` 取反。
例如：`!true == false`。
这对布尔变量同样适用。

### primitive_types2

这次没有提示 ;)

### primitive_types3

初始化某个长度的数组有一个简写方式，不用真的把 100 个元素都写出来
（当然你想写也可以！）。

例如你可以这样写：
```
let array = ["Are we there yet?"; 100];
```

加分项：还有哪些别的值能让 `a.len() >= 100` 返回 `true`？

### primitive_types4

看看书中 “Understanding Ownership -> Slices -> Other Slices” 一节：
https://doc.rust-lang.org/book/ch04-03-slices.html
用你想要放进切片里的那些元素在数组中的起始索引和结束索引（加一）来切。

如果你好奇为什么 `assert_eq!` 的第一个参数不是带 `&` 的引用，
而第二个参数却是引用，可以看看 nomicon 里关于强制转换（coercion）的章节：
https://doc.rust-lang.org/nomicon/coercions.html

### primitive_types5

看看书中 “Data Types -> The Tuple Type” 一节：
https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
尤其是讲解构（destructuring）的部分（该节倒数第二个例子）。

你需要写一个模式，把 `name` 和 `age` 绑定到元组中对应的部分。

### primitive_types6

虽然这里可以用解构式 `let` 来处理元组，但请试着改用元组索引，
书中 “Data Types -> The Tuple Type” 一节的最后一个例子有说明：
https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type
现在你的工具箱里又多了一件工具！

## 05_vecs

### vecs1

在 Rust 中，定义向量有两种方式。
1. 一种是用 `Vec::new()` 函数创建新向量，再用 `push()` 方法往里填元素。
2. 另一种是用 `vec![]` 宏，把元素写在方括号里。当你明确知道初始值时，
   这种方式更简单。

想了解更多，可以看 Rust 书中的这一章：https://doc.rust-lang.org/book/ch08-01-vectors.html

### vecs2

在第一个函数中，我们创建一个空向量，并想往里面 push 新元素。

在第二个函数中，我们对输入的值做 map 映射，再把它们收集（collect）成向量。

两个函数都完成之后，你自己判断更喜欢哪种写法。

你觉得 Rust 开发者中更常见的写法是哪种？

## 06_move_semantics

### move_semantics1

你是在向向量 push 元素的那一行看到了
“cannot borrow `vec` as mutable, as it is not declared as mutable” 这个错误，对吧？

修复方式是添加一个关键字，而这个关键字**不是**加在 push 那一行
（也就是报错的那一行）。

试着在调用 `fill_vec()` 之后访问一下 `vec0`，看看会发生什么！

### move_semantics2

第一次运行这个练习时，你会看到关于 “borrow of moved value” 的错误。
在 Rust 中，当一个参数被传给函数、而且没有被显式返回时，
你就不能再使用原来那个变量了。我们称之为变量的“移动”（move）。
当我们把 `vec0` 传进 `fill_vec` 时，它被“移动”进了 `vec1`，
也就是说我们再也无法访问 `vec0` 了。

你可以另做一份 `vec0` 中数据的副本，把它传给 `fill_vec`。
这在 Rust 中叫克隆（clone）。

### move_semantics3

它和前几个练习的区别在于：`fn fill_vec` 里原本的
`let mut vec = vec;` 这一行没有了。你可以不把这行加回去，
而是加一个 `mut`，把已有的绑定直接变成可变绑定，而不是新增一行 :)

### move_semantics4

仔细推理每个可变引用处于作用域中的范围。
在取得可变引用之后立刻更新 `x` 的值，会有帮助吗？
在书中 “References and Borrowing” 一节的 “Mutable References” 部分可以读到更多：
https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references

### move_semantics5

想找答案，可以查阅书中 “References and Borrowing” 一节：
https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

第一个问题在于 `get_char` 获取了字符串的所有权。
于是 `data` 被移动了，无法再用于 `string_uppercase`。
`data` 先被移动进 `get_char`，意味着 `string_uppercase` 无法再操作这份数据。

修好这一点之后，`string_uppercase` 的函数签名也需要调整。

## 07_structs

### structs1

Rust 不止一种结构体，其实有三种，它们都用来把相关的数据打包在一起。

普通结构体（regular struct）：把相关数据存放在命名字段里的集合。

元组结构体（tuple struct）：基本上就是有名字的元组。

单元结构体（unit struct）：没有任何字段，在泛型中很有用。

这个练习里，你需要把三种结构体各补全并实现一个。
在书中可以读到更多关于结构体的内容：
https://doc.rust-lang.org/book/ch05-01-defining-structs.html

### structs2

创建结构体实例很简单，只需要给它的字段赋一些值。

不过在实例化结构体时，有一些捷径可以走。
想知道更多，可以看这本书：
https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax

### structs3

关于 `is_international`：什么样的包裹才算国际包裹？好像和它途经的地点有关？

关于 `get_fees`：这个方法多接收了一个参数，`Package` 结构体里有与之对应的字段吗？

想知道更多关于方法实现的内容，可以看这本书：
https://doc.rust-lang.org/book/ch05-03-method-syntax.html

## 08_enums

### enums1

这次没有提示 ;)

### enums2

你可以定义带不同变体的枚举，每个变体可以有不同类型的载荷，
比如匿名结构体、结构体、单个字符串、元组、没有数据等等。

### enums3

第一步，定义好枚举，让代码能无错误地通过编译。

然后，在 `process()` 中写一个 match 表达式。

注意，你需要在 match 表达式中解构一部分消息变体，才能取出变体里的值。

## 09_strings

### strings1

`current_favorite_color` 函数目前返回的是具有 `'static` 生命周期的字符串切片。
我们知道这一点，是因为这个字符串的数据就存在于我们的代码里 —— 它不来自文件、
用户输入或别的程序 —— 所以它会和我们的程序活得一样久。

但它仍然是字符串切片。在书中讲字符串的那一章里，
有一种把字符串切片转换成 `String` 的办法，
另外还有一种用 `From` trait 的办法。

### strings2

是的，只要把绑定到 `word` 的值改成字符串切片而不是 `String`，
修起来确实很容易，对吧？不过还有一个办法：在 `if` 语句里加一个字符，
就能把 `String` 强制转换成字符串切片。

顺便说一句：如果你对这种引用转换的原理感兴趣，
可以跳到书中智能指针那一章读这一部分：
https://doc.rust-lang.org/book/ch15-02-deref.html#implicit-deref-coercions-with-functions-and-methods

### strings3

字符串有很多有用的标准库函数，我们来用几个：
https://doc.rust-lang.org/std/string/struct.String.html#method.trim

关于 `compose_me`：你可以用 `format!` 宏，
也可以把字符串切片转换成自己拥有的字符串，然后随意扩展它。

关于 `replace_me`：可以看看 `replace` 方法：
https://doc.rust-lang.org/std/string/struct.String.html#method.replace

### strings4

在 `main` 函数中，把 `placeholder` 替换成 `string` 或 `string_slice`。

例如：
`placeholder("blue");`
应该写成
`string_slice("blue");`
因为 "blue" 是 `&str`，而不是 `String`。

## 10_modules

### modules1

在 Rust 中，默认一切都是私有的。但有一个关键字可以把某些东西变成公开的！

### modules2

`delicious_snacks` 模块想对外提供一个与其内部结构不同的接口
（内部结构指的是 `fruits`、`veggies` 模块和它们的常量）。
补全那两条 `use` 语句，使其与 `main` 中的用法匹配，
并找出两个常量共同缺少的那一个关键字。

在这本书里可以学到更多：
https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#re-exporting-names-with-pub-use

### modules3

`UNIX_EPOCH` 和 `SystemTime` 声明在 `std::time` 模块中。
为这两个加上一条 `use` 语句把它们引入作用域。
你可以用嵌套路径，只用一行就引入它们两个。

## 11_hashmaps

### hashmaps1

水果的数量至少要有 5 个，而且你至少要放 3 种不同的水果。

### hashmaps2

用 `HashMap` 的 `entry()` 和 `or_insert()` 方法来实现。

在书中可以学到更多：
https://doc.rust-lang.org/book/ch08-03-hash-maps.html#only-inserting-a-value-if-the-key-has-no-value

### hashmaps3

提示 1：用 `HashMap` 的 `entry()` 和 `or_default()` 方法，
        在表中还没有某支球队时插入 `TeamScores` 的默认值。

提示 2：如果某个键已经有对应条目，`entry()` 返回的值可以基于已有值进行更新。

在书中可以学到更多：
https://doc.rust-lang.org/book/ch08-03-hash-maps.html#updating-a-value-based-on-the-old-value

## quizzes

### quiz2

`+` 运算符可以把 `String` 和 `&str` 拼接起来。

## 12_options

### options1

Option 可以携带内部值的 `Some`，也可以是不带内部值的 `None`。

取出内部值有多种方式，你可以用 `unwrap`，也可以用模式匹配。
unwrap 最简单，但怎么能安全地使用它，而不至于之后直接 panic 到你脸上？

### options2

看看这些：

- https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html
- https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html

记住，`Option` 可以在 if-let 和 while-let 语句中嵌套。

例如：`if let Some(Some(x)) = y`

也可以看看 `Option::flatten`。

### options3

编译器说 `match` 语句中发生了部分移动（partial move）。
怎样才能避免它？编译器已经给出了需要的修改。

按编译器建议修改之后，读一读相关的文档页：
https://doc.rust-lang.org/std/keyword.ref.html

## 13_error_handling

### errors1

`Ok` 和 `Err` 是 `Result` 的两个变体，所以测试想表达的是：
`generate_nametag_text` 应该返回 `Result`，而不是 `Option`。

要完成这个改动，你需要：
  - 把函数签名中的返回类型改成 `Result<String, String>`，
    它可以是 `Ok(String)` 和 `Err(String)` 这两个变体
  - 把函数体中目前返回 `Some(…)` 的地方改成返回 `Ok(…)`
  - 把函数体中目前返回 `None` 的地方改成返回 `Err(错误信息)`

### errors2

一种处理方式是对 `item_quantity.parse::<i32>()` 使用 `match` 语句，
分支分别是 `Ok(something)` 和 `Err(something)`。

不过这种模式在 Rust 中非常常见，所以有 `?` 运算符，
它基本上就帮你做了那个 match 语句要做的事！

看看 “Error Handling” 这一章的这一节：
https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator

### errors3

如果其他函数都能返回 `Result`，`main` 为什么不能？
从 `main` 函数返回类似 `Result<(), ErrorType>` 这样的类型是相当常见的惯例。

那里用单元类型 `()` 是因为，成功时并不需要返回任何实际内容。

### errors4

`PositiveNonzeroInteger::new` 总是创建新实例并返回 `Ok`。
但它应该做一些检查：检查失败时返回 `Err`，只有检查确认一切都没问题时
才返回 `Ok` :)

### errors5

`main` 函数内部可能产生两种不同的 `Result` 类型，它们通过 `?` 运算符向上传播。
我们怎样为 `main` 函数声明一个能同时容纳这两者的返回类型？

在底层，`?` 运算符会对错误值调用 `From::from`，把它转换成装箱的 trait 对象，
也就是 `Box<dyn Error>`。这个装箱的 trait 对象是多态的，
而由于所有错误都实现了 `Error` trait，
我们就能在一个 `Box` 对象里装下许多不同的错误。

看看书中的这一节：
https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator

更多关于装箱错误（boxing errors）的内容：
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/boxing_errors.html

更多关于把 `?` 运算符用在装箱错误上的内容：
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html

### errors6

这个练习使用了前面练习中已完成的 `PositiveNonzeroInteger`。

在 `TODO` 要求你修改的那一行下面，有一个例子：
在 `Result` 上使用 `map_err()` 方法把一种错误转换成另一种错误。
试着对 `parse()` 得到的 `Result` 用类似的做法，
然后就可以用 `?` 运算符提前返回了。

更多关于 `map_err()` 的内容见 `std::result` 文档：
https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err

## 14_generics

### generics1

Rust 中的向量使用泛型来创建任意类型的动态长度数组。
如果向量 `numbers` 的类型是 `Vec<T>`，那我们只能向它 push `T` 类型的值。
通过在 push 之前调用 `into()`，我们要求编译器把 `n1` 和 `n2` 转换成 `T`。
但编译器还不知道 `T` 是什么，需要一个类型标注。

`u8` 和 `i8` 都能转换成 `i16`、`i32` 和 `i64`。
为这个向量的泛型参数选一个。

### generics2

书中相关章节：
https://doc.rust-lang.org/book/ch10-01-syntax.html#in-method-definitions

## 15_traits

### traits1

关于 trait 的更多内容见这本书：
https://doc.rust-lang.org/book/ch10-02-traits.html

`+` 运算符可以把 `String` 和 `&str` 拼接起来。

### traits2

注意这个 trait 获取了 `self` 的所有权并返回 `Self`。

虽然 trait 中 `append_bar` 的签名以 `self` 作为参数，
但实现中可以改成接收 `mut self`。
之所以可以这样，是因为这个值本来就被拥有了。

### traits3

trait 可以为函数提供默认实现。
实现了这个 trait 的数据类型，如果选择不自己实现该函数，
就可以使用这些函数的默认版本。

书中相关章节：
https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations

### traits4

你可以用 trait 来代替具体类型作为参数。
试着把 `???` 换成 `impl [这里填什么？]`。

书中相关章节：
https://doc.rust-lang.org/book/ch10-02-traits.html#traits-as-parameters

### traits5

要保证一个参数实现多个 trait，可以使用 `+` 语法。
试着把 `???` 换成 `impl [这里填什么？] + [这里填什么？]`。

书中相关章节：
https://doc.rust-lang.org/book/ch10-02-traits.html#specifying-multiple-trait-bounds-with-the--syntax

## quizzes

### quiz3

想找到这个问题的最佳解法，你需要回忆关于 trait 的知识，
尤其是“trait bound 语法”：
https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax

为 impl 块指定 trait bound 的写法是：
`impl<T: Trait1 + Trait2 + …> for Foo<T> { … }`

你可能会需要这个：
`use std::fmt::Display;`

## 16_lifetimes

### lifetimes1

让编译器引导你。如果需要帮助，也看看这本书：
https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

### lifetimes2

记住，泛型生命周期 `'a` 会取一个具体的生命周期，
它等于 `x` 和 `y` 中较短的那个生命周期。

在保留内层代码块的前提下，至少有两种办法可以达到期望的结果：
1. 挪动 `string2` 的声明位置，让它和 `string1` 活得一样久
   （`result` 是怎么声明的？）
2. 把 `println!` 移进内层代码块

### lifetimes3

让编译器引导你 :)

## 17_tests

### tests1

`assert!` 是一个宏，它需要一个参数。根据参数的值，
`assert!` 要么什么都不做（这时测试通过），要么 panic（这时测试失败）。

所以试着给 `assert!` 传不同的值，看看哪些能编译、哪些通过、哪些失败 :)

如果你想检查 `false`，可以用 `!` 对你检查的结果取反，例如 `assert!(!…)`。

### tests2

`assert_eq!` 是一个接收两个参数并比较它们的宏。
试着传给它两个相等的值！也试着传给它两个不同的值！
再试着把第一个参数和第二个参数调换一下！

### tests3

我们期望 `Rectangle::new` 方法在遇到负数值时 panic。

为了处理这一点，你需要给测试函数加一个特殊的属性。

可以参考文档：
https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic

## 18_iterators

### iterators1

迭代器会遍历集合中的所有元素，但如果元素用完了呢？这时我们应该期望什么？
如果卡住了，可以看看 https://doc.rust-lang.org/std/iter/trait.Iterator.html

### iterators2

`capitalize_first`：

变量 `first` 是一个 `char`。需要把它转成大写，
再和 `chars` 中剩余的字符拼在一起，才能返回正确的 `String`。

`chars` 中剩余的字符可以用 `as_str` 方法当作字符串切片来查看。

`char` 的文档里有很多有用的方法：
https://doc.rust-lang.org/std/primitive.char.html

用 `char::to_uppercase`，它返回一个可以转换成 `String` 的迭代器。

`capitalize_words_vector`：

从切片创建迭代器。对迭代出的值应用 `capitalize_first` 函数进行转换。
记得对迭代器调用 `collect`。

`capitalize_words_string`：

这和前面的解法惊人地相似。`collect` 非常强大也非常通用，
Rust 只需要知道你想要的目标类型。

### iterators3

`divide` 函数需要在除数为 0、或者无法整除时返回正确的错误。

`division_results` 变量需要被收集成某种集合类型。

`result_with_list` 函数需要返回一个 `Result`，
成功时是整数向量，失败时是 `DivisionError`。

`list_of_results` 函数需要返回一个由 Result 组成的向量。

关于 `collect()` 里如何使用 `FromIterator` trait，见
https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.collect
这个 trait 真的非常强大！它能让这个练习的解法简单很多。

### iterators4

在命令式语言里，你可能会写一个 `for` 循环来更新一个可变变量，
或者用递归配合 match 分支来写。而在 Rust 中，
你可以采用另一种函数式思路，用 range 和迭代器优雅地计算阶乘。

看看 `fold` 和 `rfold` 方法！

### iterators5

`std::iter::Iterator` trait 的文档里有很多在这里会有帮助的方法。

`count_collection_iterator` 中的 `collection` 变量是由 `HashMap` 组成的切片。
要使用迭代器方法，需要先把它转换成迭代器。

`count_collection_iterator` 函数中 `fold` 方法会很有用。

想进一步挑战的话，可以查阅 `Iterator` 的文档，
找一个能让你的代码比用 `fold` 更简洁的方法。

## 19_smart_pointers

### box1

编译器的信息应该有帮助：处理递归类型时，我们无法存储实际类型的值，
所以需要存储一个指向它的引用（指针）。

因此我们应该把 `List` 放进一个 `Box` 里。更多细节见这本书：
https://doc.rust-lang.org/book/ch15-01-box.html#enabling-recursive-types-with-boxes

创建空列表应该相当直接（提示：读一读测试）。

对于非空列表，记住我们要用 `Cons` 这个列表构造器。
虽然当前列表存放的是整数（`i32`），
你也可以随意修改定义去试试别的类型！

### rc1

这是一个使用 `Rc<T>` 类型的简单练习。
每个 `Planet` 都拥有 `Sun`，并用 `Rc::clone()` 来增加 `Sun` 的引用计数。

在用 `drop()` 把各个 `Planet` 逐个移出作用域之后，引用计数会下降。

最后，`Sun` 又只剩下一个引用，也就是它自己。

更多内容见：https://doc.rust-lang.org/book/ch15-04-rc.html

遗憾的是，冥王星已经不再被当作行星了 :(

### arc1

把 `shared_numbers` 做成基于 `numbers` 向量的 `Arc`。
然后，为了避免复制 `numbers`，你需要在循环内部创建 `child_numbers`，
但依然要在主线程中创建。

`child_numbers` 应该是这个数字 `Arc` 的一份克隆，
而不是每个线程各自一份数字副本。

如果你理解了底层概念，这就是个很简单的练习；
但如果它让你太吃力，可以考虑读一读书中第 16 章的全部内容：
https://doc.rust-lang.org/book/ch16-00-concurrency.html

### cow1

如果 `Cow` 已经拥有这份数据，那么调用 `to_mut()` 时它就不需要克隆。

看看 `Cow` 类型的文档：
https://doc.rust-lang.org/std/borrow/enum.Cow.html

## 20_threads

### threads1

`JoinHandle` 是创建线程时返回的结构体：
https://doc.rust-lang.org/std/thread/fn.spawn.html

多线程应用的一个挑战是：主线程可能在创建出来的线程结束之前就结束了。
https://doc.rust-lang.org/book/ch16-01-threads.html#waiting-for-all-threads-to-finish-using-join-handles

用 `JoinHandle` 等待每个线程结束，并收集它们的结果。

https://doc.rust-lang.org/std/thread/struct.JoinHandle.html

### threads2

`Arc` 是一个原子引用计数指针，它允许对**不可变**数据进行安全的共享访问。
但我们想要*修改* `jobs_done` 的数量，
所以还需要另一个类型来保证同一时间只有一个线程能修改数据。
看看书中的这一节：
https://doc.rust-lang.org/book/ch16-03-shared-state.html#atomic-reference-counting-with-arct

想要更多提示就继续往下读 :)

现在你的 `main` 开头是不是已经有一个 `Arc<Mutex<JobStatus>>` 了？像这样：
```
let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
```

和书中下面这个例子里的代码类似：
https://doc.rust-lang.org/book/ch16-03-shared-state.html#sharing-a-mutext-between-multiple-threads

### threads3

处理线程之间并发的另一种方式是使用 `mpsc`
（multiple producer, single consumer，多生产者单消费者）通道来通信。

有了发送端和接收端，就可以在一个线程里发送值、在另一个线程里接收值。

通过用 `clone()` 复制原来的发送端，就可以拥有多个生产者。

书中相关章节：
https://doc.rust-lang.org/book/ch16-02-message-passing.html

## 21_macros

### macros1

调用宏时，比调用普通函数需要多写一点特别的东西。

### macros2

就“什么东西在什么位置可用”这一点来说，宏并不完全遵守 Rust 其他部分的规则。

和 Rust 中的其他东西不同，“你在哪里定义宏”和“你在哪里使用它”的顺序
确实是重要的。

### macros3

要在模块之外使用某个宏，你需要对这个模块做一点特殊的处理，
把这个宏提升（lift）到它的父级作用域中。

### macros4

只需要加一个字符就能让它编译通过。

宏的写法决定了它需要在每个“宏分支”之间看到某个东西，才能把它们分隔开。

这里的宏练习就到这里，但这只是 Rust 宏能力的冰山一角。
想要更系统的介绍，可以读一读
“The Little Book of Rust Macros”（Rust 宏小书）：
https://veykril.github.io/tlborm/

## 22_clippy

### clippy1

Rust 标准库中保存了一些长精度或无限精度数学常量的最高精度版本：
https://doc.rust-lang.org/stable/std/f32/consts/index.html

我们可能会忍不住自己写某些数学常量的近似值，
但 clippy 把这些不精确的数学常量识别为潜在的错误来源。

看看编译输出中 Clippy 警告给出的建议，
并使用 `std::f32::consts` 中合适的常量来替换。

### clippy2

对 `Option` 值使用 `for` 循环，更清晰的方式是写成 `if-let` 语句。

这不是解决本练习所必需的，但如果你对“什么时候遍历 `Option` 有用”感兴趣，
可以读文档里的这一节：
https://doc.rust-lang.org/std/option/#iterating-over-option

### clippy3

这次没有提示！

## 23_conversions

### using_as

用 `as` 运算符把 `average` 函数最后一行里的某个操作数
转换成期望的返回类型。

### from_into

按照 `From` 实现前面给出的步骤来做。

### from_str

`FromStr` 的实现应该返回一个包含 `Person` 对象的 `Ok`，
或者在字符串不合法时返回一个 `Err`。

这几乎和前面的 `from_into` 练习一样，只是它返回错误，
而不是回退到默认值。

再给一个提示：你可以用 `Result` 的 `map_err` 方法，
配合一个函数或闭包来包装 `parse::<u8>` 产生的错误。

还有一个提示：如果你想在解法里用 `?` 运算符传播错误，
可以看看
https://doc.rust-lang.org/stable/rust-by-example/error/multiple_error_types/reenter_question_mark.html

### try_from_into

标准库里有没有一个 `TryFrom` 实现，
既能完成所需的整数转换，又能检查输入的范围？

挑战：你能让这些 `TryFrom` 实现泛化到多种整数类型吗？

### as_ref_mut

给这些函数加上 `AsRef<str>` 或 `AsMut<u32>` 作为 trait bound。
