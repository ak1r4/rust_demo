fn starts_with_vowel(s: &String) -> bool {
    let vowels = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    if let Some(first_char) = s.chars().next() {
        if vowels.contains(&first_char) {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub fn get_pig_latin(s: String) -> String {
    if starts_with_vowel(&s) {
        return format!("{}-hay", s);
    } else {
        return format!("{}-{}ay", s[1..].to_string(), s[0..1].to_string());
    }
}
