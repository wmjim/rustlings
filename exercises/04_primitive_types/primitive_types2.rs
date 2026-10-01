// 字符（`char`）

fn main() {
    // 注意这里是_单_引号，它和你之前见到的双引号不一样。
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    // TODO: 仿照上面的例子，在下面声明一个名为 `your_character` 的变量，
    // 它的值是你最喜欢的字符。
    // 可以试试字母，试试数字（要用单引号括起来），试试特殊符号，
    // 试试你自己的语言之外的字符，再试试 emoji 😉
    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
