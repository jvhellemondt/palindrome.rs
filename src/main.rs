fn main() {
    println!("Hello, world!");
}

struct PalindromeChecker {}

impl PalindromeChecker {
    fn is_a_palindrome(word: &str) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_able_to_tell_that_mom_is_a_palindrome() {
        assert_eq!(PalindromeChecker::is_a_palindrome("mom"), true);
    }

    #[test]
    fn it_should_be_able_to_tell_that_bill_is_not_a_palindrome() {
        assert_eq!(PalindromeChecker::is_a_palindrome("bill"), false);
    }
}
