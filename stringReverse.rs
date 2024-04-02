fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

fn main() {
    let input_str = "Hello, world!";
    let reversed_str = reverse_string(input_str);
    println!("Reversed string: {}", reversed_str);
}
