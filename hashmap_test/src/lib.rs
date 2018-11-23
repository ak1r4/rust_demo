use std::collections::HashMap;

pub fn test_hash_map() {
    let teams: Vec<String> = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores: Vec<u64> = vec![10, 20];

    let scores: HashMap<&String, &u64> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);
}

pub fn test_hash_map_1() {
    let mut scores: HashMap<String, u64> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);
    scores.insert(String::from("White"), 30);

    println!("{:?}", scores);

    let teamname = "K".to_string();
    let score = scores.get(&teamname);
    if let Some(s) = score {
        println!("score for {} is {}", teamname, s);
    } else {
        println!("Cannot find score for {}", teamname);
    }

    for (k, v) in &scores {
        println!("{} = {}", k, v);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn hash_map() {
        super::test_hash_map();
    }

    #[test]
    fn hash_map_1() {
        super::test_hash_map_1();
    }
}
