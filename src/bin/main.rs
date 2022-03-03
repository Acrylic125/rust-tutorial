// use tutorial::collections::vector;
// use tutorial::collections::maps;
// use tutorial::errors;
// use tutorial::generics;
// use tutorial::traits;
use tutorial::lifetimes;

fn main() {
    // lifetimes::try_lifetimes();
    let person = Person {
        name: String::from("John"),
        age: 3,
    };
    println!("{}", person.name);
}

struct Person {
    name: String,
    age: i32
}