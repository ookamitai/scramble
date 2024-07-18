#[derive(Clone, PartialEq, Eq)]
pub(crate) struct ColoredChar {
    _char: char,
    _prefix: String,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ColoredText {
    _string: String, 
    _prefix: String,
}

pub mod colors {
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";

    pub const BG_BLACK: &str = "\x1b[40m";
    pub const BG_RED: &str = "\x1b[41m";
    pub const BG_GREEN: &str = "\x1b[42m";
    pub const BG_YELLOW: &str = "\x1b[43m";
    pub const BG_BLUE: &str = "\x1b[44m";
    pub const BG_MAGENTA: &str = "\x1b[45m";
    pub const BG_CYAN: &str = "\x1b[46m";
    pub const BG_WHITE: &str = "\x1b[47m";

    pub const RESET: &str = "\x1b[0m";
}

impl ColoredText {
    pub fn contents(&self) -> &String {
        &self._string
    }

    pub fn prefix(&self) -> &String {
        &self._prefix
    }

    pub fn new_plain(c: String) -> Self {
        let s = ColoredText {
            _string: c,
            _prefix: colors::RESET.to_string(),
        };
        s
    }

    pub fn new(c: String, p: String) -> Self {
        let s = ColoredText {
            _string: c,
            _prefix: p,
        };
        s
    }
}

impl ColoredChar {
    pub fn contents(&self) -> &char {
        &self._char
    }

    pub fn prefix(&self) -> &String {
        &self._prefix
    }

    pub fn new(c: char, p: String) -> Self {
        let s = ColoredChar {
            _char: c,
            _prefix: p,
        };
        s
    }
}
