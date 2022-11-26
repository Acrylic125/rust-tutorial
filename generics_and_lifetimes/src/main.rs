use std::fmt::Display;

fn main() {
    // let mut lifetime1: String;

    // {
    //     let lifetime2 = String::from("Hello");
    //     lifetime1 = lifetime2;
    // }

    // lifetime1 = String::from("Hello");
    // println!("{}", lifetime1);

    // println!("{}", longer("Hello", "World"));
    let mut ll = LinkedList::new(23);
    ll.add(2);
    ll.say_hello();
}

fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

struct LinkedListNode<T> {
    value: T,
    next: Option<Box<LinkedListNode<T>>>,
}

struct LinkedList<T> {
    head: LinkedListNode<T>,
}

impl<T> LinkedList<T> {
    fn new(head: T) -> Self {
        Self {
            head: LinkedListNode {
                value: head,
                next: None,
            },
        }
    }

    fn add(&mut self, value: T) {
        let mut current = &mut self.head;
        loop {
            match current.next {
                Some(ref mut next) => current = next,
                None => {
                    current.next = Some(Box::new(LinkedListNode { value, next: None }));
                    break;
                }
            }
        }
    }
}

impl<T: Display> LinkedList<T> {
    fn say_hello(&self) {
        let mut current = &self.head;
        loop {
            println!("{}", current.value);
            match current.next {
                Some(ref next) => {
                    current = next;
                }
                None => break,
            }
        }
    }
}
