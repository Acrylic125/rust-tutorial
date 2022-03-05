use std::ops::Deref;

pub fn try_smart_pointers() {
    try_boxing();
    try_referencing();
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

pub fn try_referencing() {
    let hello = String::from("hello");
    let ref_to_hello = &hello;
    let boxed_value: CustomBox<String> = CustomBox(String::from("hello"));

    // assert_eq!(hello, ref_to_hello); can't compare `String` with `&String` E0277
    assert_eq!(hello, *ref_to_hello);
    assert_eq!(hello, *boxed_value);
}