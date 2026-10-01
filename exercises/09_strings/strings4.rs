// 对这个函数的调用应该被替换成对 `string_slice` 或 `string` 的调用。
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: 这里有一堆值，其中有些是 `String`，有些是 `&str`。
// 你的任务是：根据你对每个值类型的判断，把 `placeholder(…)` 替换成
// `string_slice(…)` 或 `string(…)`。
fn main() {
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder("nice weather".into());

    placeholder(format!("Interpolation {}", "Station"));

    // 警告：这里是按字节索引，而不是按字符索引。
    // 按字符索引可以用 `s.chars().nth(INDEX)` 实现。
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
