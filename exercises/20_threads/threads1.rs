// 这个程序会创建多个线程，每个线程至少运行 250ms，
// 并返回它完成所需的耗时。程序应该等待所有创建出来的线程结束，
// 并把它们的返回值收集到一个向量里。

use std::{
    thread,
    time::{Duration, Instant},
};

fn main() {
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("Thread {i} done");
            start.elapsed().as_millis()
        });
        handles.push(handle);
    }

    let mut results = Vec::new();
    for handle in handles {
        // TODO: 把所有线程的结果收集到 `results` 向量里。
        // 使用 `thread::spawn` 返回的 `JoinHandle` 结构体。
    }

    if results.len() != 10 {
        panic!("Oh no! Some thread isn't done yet!");
    }

    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("Thread {i} took {result}ms");
    }
}
