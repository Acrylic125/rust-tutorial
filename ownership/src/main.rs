fn main() {
    let moveable = String::from("Hello Moveable!");
    let mut mutatable = String::from("hello");

    mv_here(moveable);
    mut_here(&mut mutatable);

    println!("{}", mutatable);
}

fn mut_here(x: &mut String) {
    x.push_str(" Mutatable!");
}

fn mv_here(x: String) {
    println!("Moved {x}")
}
