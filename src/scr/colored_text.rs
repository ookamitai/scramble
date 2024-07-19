/// A char with a desired color.
/// 
/// Stores a `char` and a string containing the desired color.
/// 
/// ## Fields
/// - `_char: char`: A simple char.
/// - `_prefix: String`: A string containing color information.
#[derive(Clone, PartialEq, Eq)]
pub struct ColoredChar {
    _char: char,
    _prefix: String,
}

/// A string with a desired color.
/// 
/// Stores a `String` and a string containing the desired color.
/// 
/// ## Fields
/// - `_string: String`: A simple string.
/// - `_prefix: String`: A string containing color information.
/// ## Example
/// ```
/// let mut scr = scramble::Scr::new();
/// let mut x: usize = 0;
/// scr.set_text(&mut x, 0, &scramble::ColoredText::new("hello, world!".to_string(), scramble::colors::RED.to_string()))
///    .update();
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct ColoredText {
    _string: String, 
    _prefix: String,
}

/// Pre-defined color constants for user convenience.
/// 
/// ## Example
/// ```
/// let mut scr = scramble::Scr::new();
/// let mut x: usize = 0;
/// scr.set_text(&mut x, 0, &scramble::ColoredText::new("hello, world!".to_string(), scramble::colors::RED.to_string()))
///    .update();
/// ```
#[allow(unused_variables)]
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
    /// Gets the contents of a `ColoredText` object.
    /// 
    /// ## Returns
    /// A String reference (`&String`) to the contents.
    pub fn contents(&self) -> &String {
        &self._string
    }

    /// Gets the prefix of a `ColoredText` object.
    /// 
    /// ## Returns
    /// A String reference (`&String`) to the prefix.
    pub fn prefix(&self) -> &String {
        &self._prefix
    }

    /// Creates a new `ColoredText` object with white text and black background.
    /// 
    /// ## Parameters
    /// - `c: String`: The contents of this `ColoredText` object.
    /// 
    /// ## Returns
    /// A `ColoredText` object.
    pub fn new_plain(c: String) -> Self {
        ColoredText {
            _string: c,
            _prefix: colors::RESET.to_string(),
        }
    }

    // Creates a new `ColoredText` object.
    /// 
    /// ## Parameters
    /// - `c: String`: The contents of this `ColoredText` object.
    /// - `p: String`: The desired color.
    /// 
    /// ## Returns
    /// A `ColoredText` object.
    pub fn new(c: String, p: String) -> Self {
        ColoredText {
            _string: c,
            _prefix: p,
        }
    }
}

impl ColoredChar {
    /// Gets the contents of a `ColoredChar` object.
    /// 
    /// ## Returns
    /// A String reference (`&String`) to the contents.
    pub fn contents(&self) -> &char {
        &self._char
    }

    /// Gets the prefix of a `ColoredChar` object.
    /// 
    /// ## Returns
    /// A String reference (`&String`) to the prefix.
    pub fn prefix(&self) -> &String {
        &self._prefix
    }

    // Creates a new `ColoredChar` object.
    /// 
    /// ## Parameters
    /// - `c: char`: The contents of this `ColoredChar` object.
    /// - `p: String`: The desired color.
    /// 
    /// ## Returns
    /// A `ColoredChar` object.
    pub fn new(c: char, p: String) -> Self {
        ColoredChar {
            _char: c,
            _prefix: p,
        }
    }
}
