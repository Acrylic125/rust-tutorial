use std::fmt::Display;
use std::ops::Deref;

pub fn try_smart_pointers() {
    try_boxing();
    try_referencing();
    try_dropping();
}

enum ConsList<T> {
    Next(T, Box<ConsList<T>>),
    Nil
}

pub fn try_boxing() {
    let cons_list: ConsList<i32> =
        ConsList::Next(1, Box::new(
            ConsList::Next(2, Box::new(
                ConsList::Next(3, Box::new(
                    ConsList::Next(4, Box::new(
                        ConsList::Next(5, Box::new(
                            ConsList::Next(6, Box::new(
                                ConsList::Next(7, Box::new(ConsList::Nil))
                            ))
                        ))
                    ))
                ))
            ))
        ));
}

pub struct CustomBox<T>(T);

impl<T> Deref for CustomBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> Drop for CustomBox<T> {
    fn drop(&mut self) {}
}

pub struct DropBox<T: Display>(T);

impl<T: Display> Drop for DropBox<T> {
    fn drop(&mut self) {
        println!("Dropped {}", &self.0);
    }
}

pub fn try_referencing() {
    let hello = String::from("hello");
    let ref_to_hello = &hello;
    let boxed_value: CustomBox<String> = CustomBox(String::from("hello"));

    // assert_eq!(hello, ref_to_hello); can't compare `String` with `&String` E0277
    assert_eq!(hello, *ref_to_hello);
    assert_eq!(hello, *boxed_value);
}

pub fn try_dropping() {
    let box_1 = DropBox(1);
    let box_2 = DropBox(2);
    let box_3 = DropBox(3);
    let box_4 = DropBox(4);
    let box_5 = DropBox(5);

    drop(box_3);
}