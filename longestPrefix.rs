fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = String::new();
    let first_string = &strings[0];

    'outer: for (i, char) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(c) = string.chars().nth(i) {
                if c != char {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        prefix.push(char);
    }

    prefix
}

fn main() {
    let strings1 = vec!["flower".to_string(), "flow".to_string(), "flight".to_string()];
    let strings2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];

    println!("Longest common prefix: {}", longest_common_prefix(&strings1));
    println!("Longest common prefix: {}", longest_common_prefix(&strings2));
}
