// TODO: Add the missing type of the argument `num` after the colon `:`.
// 每个函数参数都必须标注类型
fn call_me(num: u64) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
