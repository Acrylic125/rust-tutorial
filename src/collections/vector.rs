pub fn try_vector() {
    let mut vec: Vec<i32> = Vec::with_capacity(3);
    vec.push(1);
    vec.push(2);
    vec.push(3);

    for ele in vec {
        println!("{}", ele);
    }
    println!("Trying vector!");
}

pub fn try_vector_with_predefined() {
    let vec = vec![1, 2, 3, 4, 5];
    for ele in vec {
        println!("{}", ele);
    }
}

pub fn try_getting_value() {
    let vec = vec![1, 2, 3, 4, 5];
    println!("Directly Retrieve {}", &vec[2]);

    match vec.get(31121) {
        Some(value) => println!("From get(): {}", value),
        None => println!("Failed from get()!"),
    };
}

pub fn create_dummy_vector() -> Vec<i32> {
    vec![1, 2, 3, 4, 5]
}