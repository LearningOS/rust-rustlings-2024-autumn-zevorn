// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a

/// 使用生命周期注解`'a`来确保引用的生命周期是兼容的，防止悬挂引用。
/// 这里的`'a`表示一个泛型生命周期参数，确保`author`和`title`的生命周期不会超过指定的生命周期范围。
struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book { author: &name, title: &title };

    println!("{} by {}", book.title, book.author);
}
