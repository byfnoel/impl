fn main() {
    let result = Palindrome::is_palindrome("madam");
    println!("{}", result);
}

struct Palindrome {}

impl Palindrome {
    fn is_palindrome(input: &str) -> bool {
        let mut left = 0;
        let mut right = input.len() - 1;
        let mut valid = false;
        while left < right {
            if input.chars().nth(left).unwrap() != input.chars().nth(right).unwrap() {
                valid = false;
                break;
            }
            left += 1;
            right -= 1;
        }
        return valid;
    }
}
