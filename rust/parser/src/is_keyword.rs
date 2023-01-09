pub fn is_keyword(value: &str) -> bool {
    matches!(value, "if" | "do" | "else" | "and" | "or")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retun_true_on_keyword() {
        assert!(is_keyword("else"));
    }

    #[test]
    fn return_false_on_nonkeyword() {
        assert!(!is_keyword("notakeyword"));
    }
}
