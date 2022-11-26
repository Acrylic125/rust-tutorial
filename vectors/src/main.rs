fn main() {
    let mut vec: Vec<String> = Vec::new();

    vec.push(String::from("Hello"));
    vec.push(String::from("World"));
    vec.push(String::from("Rust"));

    for i in &vec {
        println!("{}", i);
    }
    vec.clear();
}
