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

impl Keyboard {
    pub fn new() -> Self {
        Self {
            keyboard: vec![vec![]],
        }
    }

    /// Add a text callback to keyboard
    pub fn text(&mut self, text: &String, callback: &String) -> InlineKeyboardMarkup {
        self.keyboard
            .last_mut()
            .unwrap()
            .push(InlineKeyboardButton::callback(text, callback));

        self.get()
    }

    /// Add an url button to keyboard
    pub fn url(&mut self, text: &String, url: &String) -> InlineKeyboardMarkup {
        let parsed_url = Url::parse(&url).unwrap();

        self.keyboard
            .last_mut()
            .unwrap()
            .push(InlineKeyboardButton::url(text, parsed_url));

        self.get()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json() {
        let mut keyboard = Keyboard::new();
        keyboard.url(
            &"Shaxsiy Chat".to_string(),
            &"https://t.me/rustlanguz".to_string(),
        );

        assert_eq!(
            serde_json::to_string(&keyboard.get()).unwrap(),
            "{\"inline_keyboard\":[[{\"text\":\"Shaxsiy Chat\",\"url\":\"https://t.me/rustlanguz\"}]]}"
        );
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
