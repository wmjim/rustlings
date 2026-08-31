fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // 模式匹配解构元组值
    let /* your pattern here */(name, age) = cat;

    println!("{name} is {age} years old");
}
