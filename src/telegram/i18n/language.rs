use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Language {
    translations: HashMap<String, String>,
}

impl Language {
    pub fn new() -> Self {
        Language::default()
    }

    pub fn parse_translation(&mut self, file: &str) {
        let content = std::fs::read_to_string(file).unwrap();

        let translations: HashMap<String, String> = toml::from_str(&content).unwrap();

        for (key, value) in translations {
            self.add_translation(key.to_string(), value.as_str().to_string());
        }
    }

    pub fn add_translation(&mut self, key: String, value: String) {
        self.translations.insert(key, value);
    }

    pub fn get_translation(&self, key: &str) -> Option<&String> {
        self.translations.get(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_kv() {
        let mut language = Language::new();
        language.add_translation("hello".to_string(), "Hello, World!".to_string());

        let translation = language.get_translation("hello");

        // Test
        assert_eq!(translation, Some(&"Hello, World!".to_string()));
    }

    #[test]
    fn test_language_read_file() {
        let mut language = Language::new();
        language.parse_translation("./locales/en.toml");

        let translation = language.get_translation("lorem");

        // Test
        assert_eq!(translation, Some(&"ipsum shit".to_string()));
    }
}
