use std::fmt::Debug;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use url::Url;

#[derive(Clone, Debug, PartialEq)]
pub struct Keyboard {
    keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Keyboard::new()
    }
}

pub enum KeyboardError {
    URLParseError,
}

impl From<url::ParseError> for KeyboardError {
    fn from(_: url::ParseError) -> Self {
        KeyboardError::URLParseError
    }
}

impl Debug for KeyboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyboardError::URLParseError => write!(f, "Provided URL is not valid"),
        }
    }
}

impl Keyboard {
    /// Initialize an empty keyboard
    pub fn new() -> Self {
        Self {
            keyboard: vec![vec![]],
        }
    }

    /// Add a button to keyboard
    pub fn add(&mut self, button: InlineKeyboardButton) {
        match self.keyboard.last_mut() {
            Some(row) => row.push(button),
            None => self.keyboard.push(vec![button]),
        }
    }

    /// Add a text callback to keyboard
    pub fn text<T>(&mut self, text: T, callback: T) -> InlineKeyboardMarkup
    where
        T: ToString,
    {
        self.add(InlineKeyboardButton::callback(
            text.to_string(),
            callback.to_string(),
        ));
        self.get()
    }

    /// Add an url button to keyboard
    pub fn url<T>(&mut self, text: T, url: T) -> Result<InlineKeyboardMarkup, KeyboardError>
    where
        T: ToString,
    {
        let parsed_url = match Url::parse(&url.to_string()) {
            Ok(url) => url,
            Err(_) => return Err(KeyboardError::URLParseError),
        };

        self.add(InlineKeyboardButton::url(text.to_string(), parsed_url));

        Ok(self.get())
    }

    /// Add next buttons from new line
    pub fn row(&mut self) -> InlineKeyboardMarkup {
        self.keyboard.push(vec![]);
        self.get()
    }

    /// Return the final result
    pub fn get(&self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(self.keyboard.clone())
    }

    pub fn switch_inline_current(&mut self, text: &str, query: &str) -> InlineKeyboardMarkup {
        self.keyboard.last_mut().unwrap().push(
            InlineKeyboardButton::switch_inline_query_current_chat(text, query),
        );

        self.get()
    }
    
    pub fn web_app(&mut self, text: &str, link: &str) -> InlineKeyboardMarkup {
        self.keyboard
            .last_mut()
            .unwrap()
            .push(InlineKeyboardButton::web_app(
                text,
                WebAppInfo {
                    url: Url::from_str(link).unwrap(),
                },
            ));

        self.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_button() {
        let mut keyboard = Keyboard::new();
        let text = "Example Text".to_string();
        let callback_data = "callback_data".to_string();

        keyboard.text(&text, &callback_data);

        assert_eq!(
            keyboard.get(),
            InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::callback(
                text,
                callback_data
            )]])
        );
    }

    #[test]
    fn test_url_button() {
        let mut keyboard = Keyboard::new();
        let text = "Visit Rust".to_string();
        let url = "https://www.rust-lang.org".to_string();

        let _ = keyboard.url(&text, &url);

        assert_eq!(
            keyboard.get(),
            InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(
                text,
                Url::parse(&url).unwrap()
            )]])
        );
    }

    #[test]
    fn test_row_creation() {
        let mut keyboard = Keyboard::new();
        keyboard.row(); // Create a new row

        // Assert that a new row was added
        let expected_keyboard = InlineKeyboardMarkup::new(vec![vec![], vec![]]); // Empty keyboard with two rows
        assert_eq!(keyboard.get(), expected_keyboard);
    }

    #[test]
    fn test_multiple_buttons_and_rows() {
        let mut keyboard = Keyboard::new();
        let text_button = "Button 1".to_string();
        let callback_data = "callback_1".to_string();
        let url_button = "Button 2".to_string();
        let url = "https://example.com".to_string();

        // Add a text button
        keyboard.text(&text_button, &callback_data);
        // Add a new row
        keyboard.row();
        // Add a URL button in the new row
        let _ = keyboard.url(&url_button, &url);

        let expected_keyboard = InlineKeyboardMarkup::new(vec![
            vec![InlineKeyboardButton::callback(text_button, callback_data)],
            vec![InlineKeyboardButton::url(
                url_button,
                Url::parse(&url).unwrap(),
            )],
        ]);

        assert_eq!(keyboard.get(), expected_keyboard);
    }
}

// ========================
// Example
// ========================
// pub fn example_keyboard() -> InlineKeyboardMarkup {
//     let mut keyboard = Keyboard::new();
//     keyboard.url(
//         &"Shaxsiy Chat".to_string(),
//         &"https://t.me/rustlanguz".to_string(),
//     )
// }
//
// pub async fn example(bot: &Bot, msg: &Message) -> ResponseResult<()> {
//     bot.send_message(msg.chat.id, TEXT)
//         .parse_mode(ParseMode::Html)
//         .reply_markup(example_keyboard())
//         .await?;
//
//     Ok(())
// }
