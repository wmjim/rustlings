// 这个测验考查以下小节：
// - 字符串
// - 向量
// - 移动语义
// - 模块
// - 枚举
//
// 我们来用一个函数做成一台小机器。输入是一串字符串和命令，
// 命令决定要对字符串执行什么操作。操作可以是：
// - 把字符串转换为大写
// - 去掉字符串两端的空白
// - 在字符串后面追加 "bar"，追加指定的次数
//
// 具体的输入输出形式是：
// - 输入是一个由二元组组成的向量，
//   第一个元素是字符串，第二个元素是命令。
// - 输出是一个字符串向量。

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: 按上面的描述补全这个函数。
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // 你可以在这里随意试验。
}

#[cfg(test)]
mod tests {
    // TODO: 我们需要导入什么，才能让 `transformer` 进入作用域？
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
