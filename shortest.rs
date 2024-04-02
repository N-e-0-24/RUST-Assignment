fn shortest_word(sentence: &str) -> Option<&str> {
    let mut shortest: Option<&str> = None;
    
    for word in sentence.split_whitespace() {
        match shortest {
            Some(shortest_word) => {
                if word.len() < shortest_word.len() {
                    shortest = Some(word);
                }
            }
            None => {
                shortest = Some(word);
            }
        }
    }
    
    shortest
}

fn main() {
    let sentence = "The quick brown fox jumps over the lazy dog";
    match shortest_word(sentence) {
        Some(shortest) => println!("Shortest word: {}", shortest),
        None => println!("No words in the sentence"),
    }
}
