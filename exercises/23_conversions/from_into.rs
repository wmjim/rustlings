// `From` trait 用于值到值的转换。如果实现了 `From`，
// 就会自动获得一个 `Into` 的实现。
// 你可以在文档中读到更多内容：
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 我们实现 Default trait，是为了在给定的字符串无法转换成 `Person` 对象时
// 提供一个回退用的默认值。
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: 补全这个 `From` 实现，让它能从一个形如 "Mark,20" 的字符串
// 解析出 `Person`。
// 注意，你需要用类似 `"4".parse::<u8>()` 的方式把年龄部分解析成 `u8`。
//
// 步骤：
// 1. 按字符串中出现的逗号进行分割。
// 2. 如果分割得到的元素个数不是 2 个，返回 `Person` 的默认值。
// 3. 用分割出的第一个元素作为名字。
// 4. 如果名字为空，返回 `Person` 的默认值。
// 5. 把分割出的第二个元素解析成 `u8` 作为年龄。
// 6. 如果年龄解析失败，返回 `Person` 的默认值。
impl From<&str> for Person {
    fn from(s: &str) -> Self {}
}

fn main() {
    // 使用 `from` 函数。
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // 由于为 Person 实现了 `From`，我们可以使用 `Into`。
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
