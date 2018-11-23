use std::collections::HashMap;

pub fn get_mean(v: Vec<u32>) -> f32{
    let sum: u32 = v.iter().sum();
    return sum as f32 / v.len() as f32;
}

pub fn get_median(v: Vec<u32>) -> u32 {
    let size = v.len();
    return v[size / 2];
}

pub fn get_mode(v: Vec<u32>) -> u32 {
    let mut number_counter: HashMap<u32, u32> = HashMap::new();
    let mut max_count_number = 0;
    let mut max_count = 0;

    for n in &v {
        let count = number_counter.entry(*n).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            max_count_number = *n;
        }
    }

    return max_count_number;
}
