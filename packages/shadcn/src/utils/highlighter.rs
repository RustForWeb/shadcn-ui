use std::sync::LazyLock;

// Based on https://github.com/lukeed/kleur.
struct Style {
    open: String,
    close: String,
}

impl Style {
    fn new(x: u8, y: u8) -> Self {
        Self {
            open: format!("\x1b[{x}m"),
            close: format!("\x1b[{y}m"),
        }
    }

    fn format(&self, txt: &str) -> String {
        format!(
            "{}{}{}",
            self.open,
            if txt.contains(&self.close) {
                txt.replace(&self.close, &format!("{}{}", self.close, self.open))
            } else {
                txt.into()
            },
            self.close
        )
    }

    fn red() -> Style {
        Style::new(31, 39)
    }

    fn green() -> Style {
        Style::new(32, 39)
    }

    fn yellow() -> Style {
        Style::new(33, 39)
    }

    fn cyan() -> Style {
        Style::new(36, 39)
    }
}

pub static HIGHLIGHTER: LazyLock<Highlighter> = LazyLock::new(Highlighter::new);

pub struct Highlighter {
    error: Style,
    warn: Style,
    info: Style,
    success: Style,
}

impl Highlighter {
    fn new() -> Self {
        Self {
            error: Style::red(),
            warn: Style::yellow(),
            info: Style::cyan(),
            success: Style::green(),
        }
    }

    pub fn error(&self, text: &str) -> String {
        self.error.format(text)
    }

    pub fn warn(&self, text: &str) -> String {
        self.warn.format(text)
    }

    pub fn info(&self, text: &str) -> String {
        self.info.format(text)
    }

    pub fn success(&self, text: &str) -> String {
        self.success.format(text)
    }
}
