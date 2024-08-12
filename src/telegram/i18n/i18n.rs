use super::language::Language;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct I18n {
    users: HashMap<i64, String>,
    pub languages: HashMap<String, Language>,
}

impl I18n {
    pub fn new() -> Self {
        I18n::default()
    }

    pub fn load_translations(&mut self) -> () {
        let locales = std::fs::read_dir("locales").unwrap();

        for locale in locales {
            let safe = locale.unwrap();

            let path = safe.path();
            let file = path.to_str().unwrap();

            let mut language = Language::new();
            language.parse_translation(file);

            let name = file.split("/").last().unwrap().split(".").next().unwrap();

            self.languages.insert(name.to_owned(), language);
        }
    }

    pub fn add_user(&mut self, user_id: i64, language: &str) {
        self.users.insert(user_id, language.to_string());
    }

    pub fn get_translation(&self, language: &str, key: &str) -> Option<&String> {
        self.languages.get(language)?.get_translation(key)
    }

    pub fn get_user_translation(&self, user_id: i64, key: &str) -> Option<&String> {
        let language = self.users.get(&user_id)?;
        self.get_translation(language, key)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_i18n_init() {
        // Create instance and load locales
        let mut i18n = I18n::new();
        i18n.load_translations();

        // Get a translation
        let translation = i18n.get_translation("en", "lorem");

        // Test
        assert_eq!(translation, Some(&"ipsum shit".to_string()));
    }

    #[test]
    fn test_user() {
        // Create instance and load locales
        let mut i18n = I18n::new();
        i18n.load_translations();

        //
        i18n.add_user(123123123123, "en");

        // Let's try sending request
        let translation = i18n.get_user_translation(123123123123, "lorem");

        // Test
        assert_eq!(translation, Some(&"ipsum shit".to_string()));
    }
}
