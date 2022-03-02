pub fn try_lifetimes() {
    let static_variable: &'static str = "Hello";

    let s1 = "Hello";
    let s2 = "Hello 2";

    let longest = longest_string(s1, s2);
    println!("{}", longest);

    println!("{}", test_elision(static_variable));
}

pub fn test_elision(s: &str) -> &str {
    println!("{}", s);
    s
}

// pub fn test_elision_dnw(s: &str, a: &str) -> &str {
//     println!("{}", s);
//     s
// }

pub fn longest_string<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
