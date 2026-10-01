// 这个练习是 `errors4` 的修改版。它用到了一些课程后面才会讲的概念，
// 比如 `Box` 和 `From` trait。现在不必深入理解它们，如果你想的话可以提前看看。
// 眼下你可以把 `Box<dyn ???>` 类型理解为“我想要任何能做 ??? 的东西”。
//
// 简单来说，box 的这种用法适用于这样的场景：你想持有一个值，
// 而你只关心它是一种实现了某个特定 trait 的类型。
// 为此，`Box` 要声明为 `Box<dyn Trait>` 类型，
// 其中 `Trait` 是编译器在这个上下文中对任何用到的值所查找的 trait。
// 对这个练习来说，这个上下文就是可能出现在 `Result` 中的错误。

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// 为了让 `CreationError` 能实现 `Error`，这是必需的。
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: 补上正确的返回类型 `Result<(), Box<dyn ???>>`。
// 我们可以用什么来描述这两种错误？有没有一个它们都实现了的 trait？
fn main() {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
