use std::fmt::Display;

pub fn try_closures() {
    let chat_messenger: Messenger<String> = Messenger::new(
        Box::new(|event| {
            println!("On Send: {}", event.message);
        }),
        Box::new(|event| {
            println!("On Receive: {}", event.message);
        })
    );
    chat_messenger.send(String::from("Hello, world!"));
}

pub struct MessageEvent<T>
    where T: Display
{
    pub message: T,
}

pub struct Messenger<T>
    where T: Display
{
    pub on_send: Box<dyn Fn(&MessageEvent<T>)>,
    pub on_receive: Box<dyn Fn(&MessageEvent<T>)>
}

impl<T> Messenger<T>
    where T: Display
{
    pub fn new(on_send: Box<dyn Fn(&MessageEvent<T>)>,
               on_receive: Box<dyn Fn(&MessageEvent<T>)>) -> Messenger<T> {
        Messenger {
            on_send, on_receive
        }
    }

    pub fn send(self, message: T) {
        let event = MessageEvent {
            message
        };
        (self.on_send)(&event);
        (self.on_receive)(&event);
    }
}