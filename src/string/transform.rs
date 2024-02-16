pub trait Transform {
    fn capitalize(&self) -> Self;
}

impl Transform for String {
    fn capitalize(&self) -> Self {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalize_empty_string() {
        let input = String::new();
        let transformed = input.capitalize();
        assert_eq!(transformed, "");
    }

    #[test]
    fn capitalize_lowercase_string() {
        let input = String::from("hello");
        let transformed = input.capitalize();
        assert_eq!(transformed, "Hello");
    }

    #[test]
    fn capitalize_already_capitalized_string() {
        let input = String::from("Hello");
        let transformed = input.capitalize();
        assert_eq!(transformed, "Hello");
    }

    #[test]
    fn capitalize_all_uppercase_string() {
        let input = String::from("HELLO");
        let transformed = input.capitalize();
        assert_eq!(transformed, "HELLO");
    }

    #[test]
    fn capitalize_string_with_leading_whitespace() {
        let input = String::from(" hello");
        let transformed = input.capitalize();
        assert_eq!(transformed, " hello");
    }

    #[test]
    fn capitalize_non_ascii_string() {
        let input = String::from("ñandú");
        let transformed = input.capitalize();
        assert_eq!(transformed, "Ñandú");
    }

    #[test]
    fn capitalize_string_with_numbers() {
        let input = String::from("123hello");
        let transformed = input.capitalize();
        assert_eq!(transformed, "123hello");
    }
}
