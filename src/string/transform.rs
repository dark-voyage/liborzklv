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
    fn capitalize() {
        let result = "hello".to_string().capitalize();
        assert_eq!(result, "Hello");
    }
}
