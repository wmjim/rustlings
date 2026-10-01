// 你可以用 `use` 关键字把任意位置的模块路径引入作用域，
// 尤其是标准库中的路径。

// TODO: 把 `std::time` 模块中的 `SystemTime` 和 `UNIX_EPOCH` 引入你的作用域。
// 如果能只用一行就做到，会额外加分！
// use ???;

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
