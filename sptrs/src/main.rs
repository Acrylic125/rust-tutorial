fn main() {
    println!("Hello, world!");
    ConsList::Cons(5, Box::new(ConsList::Cons(10, Box::new(ConsList::Nil))));
}

enum ConsList<T> {
    Cons(T, Box<ConsList<T>>),
    Nil,
}
