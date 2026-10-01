// 当结构体持有引用时，同样需要生命周期。

// TODO: 修复与这个结构体相关的编译错误。
struct Book {
    author: &str,
    title: &str,
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
