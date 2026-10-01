// 在这个练习中，我们有一个名为 `numbers` 的 `u32` 向量，值从 0 到 99。
// 我们希望让 8 个不同的线程同时使用这组数字。
// 每个线程负责按偏移量求每隔 8 个元素的和。
//
// 第一个线程（偏移 0）会累加 0, 8, 16, …
// 第二个线程（偏移 1）会累加 1, 9, 17, …
// 第三个线程（偏移 2）会累加 2, 10, 18, …
// …
// 第八个线程（偏移 7）会累加 7, 15, 23, …
//
// 每个线程都应该持有一个指向这个数字向量的引用计数指针。
// 但 `Rc` 不是线程安全的，因此我们需要使用 `Arc`。
//
// 不要被线程的创建和等待方式分散注意力，
// 我们会在后面讲线程的练习里专门练习这些。

// 不要修改下面的代码。
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: 用 `Arc` 定义 `shared_numbers`。
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: 基于 `shared_numbers` 定义 `child_numbers`。
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
