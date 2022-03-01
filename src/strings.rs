pub fn test_string() {
    let str = String::from("Foo");
    let str2 = String::from("Bar");

    let mut concatinated = str + &str2;
    concatinated += "!";

    println!("{}", concatinated);
}

pub fn iterate_string(str: &str) {
    for ele in str.to_string().bytes() {
        println!("{}", ele);
    }
}
