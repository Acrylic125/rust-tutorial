use std::cell::RefCell;
use std::fmt::Display;
use std::ops::Deref;
use std::rc::Rc;

pub fn try_smart_pointers() {
    try_boxing();
    try_referencing();
    try_dropping();
    try_reference_count();
    try_interior_mutability();
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

pub fn try_reference_count() {
    let value = 3;
    let reference = Rc::new(value);

    let new_reference = Rc::clone(&reference);
    println!("#1 reference {}", Rc::strong_count(&reference));
    let new_reference = Rc::clone(&reference);

    println!("#2 reference {}", Rc::strong_count(&reference));

    {
        let new_reference = Rc::clone(&reference);

        println!("#3 reference {}", Rc::strong_count(&reference));
    }
    println!("#4 reference {}", Rc::strong_count(&reference));
    let new_reference = Rc::clone(&reference);
    drop(new_reference);
    println!("#5 reference {}", Rc::strong_count(&reference));

}

struct Wrapped<T> {
    pub wrapped: RefCell<T>,
}

pub fn try_interior_mutability() {
    let wrapped = Wrapped {
        wrapped: RefCell::new(3)
    };
    *wrapped.wrapped.borrow_mut() = 31;
    println!("wrapped {}", wrapped.wrapped.borrow())
}