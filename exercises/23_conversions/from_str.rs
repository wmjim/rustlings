// 这个练习和前面的 `from_into` 类似。但这一次我们要实现 `FromStr`，
// 并且返回错误，而不是回退到默认值。
// 此外，实现 `FromStr` 之后，你就可以用字符串上的 `parse` 方法
// 生成实现者类型的对象。你可以在文档中读到更多内容：
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// 我们将在 `FromStr` 实现中使用这个错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 字段数量不正确
    BadLen,
    // 名字字段为空
    NoName,
    // 包装来自 parse::<u8>() 的错误
    ParseInt(ParseIntError),
}

// TODO: 补全这个 `FromStr` 实现，让它能从一个形如 "Mark,20" 的字符串
// 解析出 `Person`。
// 注意，你需要用类似 `"4".parse::<u8>()` 的方式把年龄部分解析成 `u8`。
//
// 步骤：
// 1. 按字符串中出现的逗号进行分割。
// 2. 如果分割得到的元素个数不是 2 个，返回错误 `ParsePersonError::BadLen`。
// 3. 用分割出的第一个元素作为名字。
// 4. 如果名字为空，返回错误 `ParsePersonError::NoName`。
// 5. 把分割出的第二个元素解析成 `u8` 作为年龄。
// 6. 如果年龄解析失败，返回错误 `ParsePersonError::ParseInt`。
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
