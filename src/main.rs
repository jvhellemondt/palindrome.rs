fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_able_to_tell_that_mom_is_a_palindrome() {
        assert_eq!(PalindromeChecker::is_a_palindrome("mom"), true);
    }
}
