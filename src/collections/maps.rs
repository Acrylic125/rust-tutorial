use std::collections::HashMap;

pub fn try_map() {
    let mut age_map: HashMap<String, u32> = HashMap::new();
    age_map.insert("John Smith".to_string(), 32);
    age_map.insert("John Swift".to_string(), 24);
    age_map.insert("Iian Lambda".to_string(), 16);
    age_map.insert("Karen White".to_string(), 8);
    age_map.insert("Aaron Brown".to_string(), 0);

    match age_map.get("John Smith") {
        Some(age) => {
            println!("{}", age);
        }
        None => {}
    }
}
