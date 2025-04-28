use std::sync::LazyLock;

use super::highlighter::HIGHLIGHTER;

pub static LOGGER: LazyLock<Logger> = LazyLock::new(Logger::new);

pub struct Logger;

impl Logger {
    fn new() -> Self {
        Self
    }

    pub fn error(&self, text: &str) {
        println!("{}", HIGHLIGHTER.error(text));
    }

    pub fn warn(&self, text: &str) {
        println!("{}", HIGHLIGHTER.warn(text));
    }

    pub fn info(&self, text: &str) {
        println!("{}", HIGHLIGHTER.info(text));
    }

    pub fn success(&self, text: &str) {
        println!("{}", HIGHLIGHTER.success(text));
    }

    pub fn log(&self, text: &str) {
        println!("{text}");
    }

    pub fn r#break(&self) {
        println!();
    }
}
