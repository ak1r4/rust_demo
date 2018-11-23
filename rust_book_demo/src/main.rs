fn main() {
    let mut s0 = String::from("test");
    println!("s0: {}, len: {}", s0, calculate_len(&s0));
    s0.push_str(", haha");
    println!("s00: {}", s0);
    change(&mut s0, "hoho".to_string());
    println!("s01: {}", s0);

    println!("first word of \"{}\": {}", "mainly happened", find_first_word(&"mainly happened".to_string()));

    // if let
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => {
            println!("Three!");
        },
        _ => (),
    }
}

fn calculate_len(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String, additional_s: String) {
    s.push_str(&additional_s);
}

fn find_first_word(s: &String) -> usize {
    let b = s.as_bytes();

    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
