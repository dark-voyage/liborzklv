use std::io::Write;

pub struct Input {
    text: String,
}

impl Default for Input {
    fn default() -> Self {
        Self::new()
    }
}

impl Input {
    pub fn new() -> Self {
        Input {
            text: String::new(),
        }
    }

    fn trim(&mut self) {
        if self.text.ends_with('\n') {
            self.text.pop();
            if self.text.ends_with('\r') {
                self.text.pop();
            }
        }
    }

    pub fn read(&mut self) -> String {
        // IO stdin catch
        std::io::stdout()
            .flush()
            .expect("Whoops, couldn't flush stdout");
        let stdin = std::io::stdin();

        // Prepend stdin to the text
        stdin
            .read_line(&mut self.text)
            .expect("Error occurred while prepending stdin");

        // Remove trailing new line
        self.trim();

        // Show the input
        println!("{}", self.text);

        // Get a clone for this guy
        self.get()
    }

    pub fn readp<T>(&mut self, prompt: T) -> String
    where
        T: AsRef<str>,
    {
        print!("{}", prompt.as_ref());
        self.read()
    }

    pub fn reset(&mut self) {
        self.text.clear();
    }

    pub fn get(&self) -> String {
        self.text.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = Input::new();
        assert_eq!(input.get(), "");
    }

    #[test]
    fn test_reset() {
        let mut input = Input::new();
        input.text = "Hello".to_string();
        input.reset();
        assert_eq!(input.get(), "");
    }

    // Make `trim` method public or #[cfg(test)] pub(crate) for testing
    #[test]
    fn test_trim() {
        let mut input = Input::new();
        input.text = "Hello\n".to_string();
        input.trim();
        assert_eq!(input.get(), "Hello");

        input.text = "Hello\r\n".to_string();
        input.trim();
        assert_eq!(input.get(), "Hello");

        input.text = "Hello".to_string();
        input.trim();
        assert_eq!(input.get(), "Hello");
    }
}
