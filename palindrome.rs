fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    let test_str1 = "racecar";
    let test_str2 = "hello";
    let test_str3 = "abba";

    println!("Is '{}' a palindrome? {}", test_str1, is_palindrome(test_str1));
    println!("Is '{}' a palindrome? {}", test_str2, is_palindrome(test_str2));
    println!("Is '{}' a palindrome? {}", test_str3, is_palindrome(test_str3));
}
