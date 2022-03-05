pub fn try_smart_pointers() {

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