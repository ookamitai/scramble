pub mod colored_text;
use std::io::Write; 
use colored_text::*;

/// A utility function to get the terminal window size.
/// 
/// _requires the `term_size` crate_
/// 
/// ## Returns 
/// - Returns (80, 24) if the operation failed, or
/// 
/// - A tuple of (usize, usize):
///   - `[0]`: width of the window  
///   - `[1]`: height of the window  
/// 
/// ## Example 
/// ```
/// let (w, h) = scramble::get_dimensions();
/// ```

pub fn get_dimensions() -> (usize, usize) {
    // returns (width, height)
    // term_size::dimensions().expect("Failed to get terminal dimensions!")
    term_size::dimensions().unwrap_or((80, 24))

}

/// A screen object.
/// 
/// _Only __one__ instance should exist at runtime to avoid racing between instances_
/// 
/// Operations directing to the instance will be buffered into `_buffer` first 
/// and will NOT reflect the changes until the `.update()` method is called. 
/// 
/// ## Fields
/// - `_buffer: Vec<Vec<ColoredChar>>`: A grid (_w * _h) filled with chars of desired colors. This is only for buffering.
/// - '_current: Vec<Vec<ColoredChar>>': A grid (_w * _h) describing the current state of the terminal window.
/// - `_w: usize, _h: usize`: The dimensions of the terminal window.
pub struct Scr {
    _buffer: Vec<Vec<ColoredChar>>,
    _current: Vec<Vec<ColoredChar>>,
    _w: usize, _h: usize,
}

impl Default for Scr {
    fn default() -> Self {
        Self::new()
    }
}

impl Scr {
    /// Gets the diemensions of this `Scr` instance.
    /// 
    /// ## Returns 
    /// A tuple of (usize, usize).  
    /// - `[0]`: width of the window  
    /// - `[1]`: height of the window
    /// 
    /// ## Example
    /// ```
    /// let scr = scramble::Scr::new();
    /// let (w, h) = scr.dimensions();
    /// ```
    pub fn dimensions(&self) -> (usize, usize) {
        (self._w, self._h)
    }
    /// Creates a new screen.
    /// 
    /// ## Returns 
    /// An `Scr` object with predetermined dimensions.
    /// 
    /// ## Examples
    /// ```
    /// let scr = scramble::Scr::new();
    /// ```
    /// 
    /// Alternatively, since this method requires no parameters and 
    /// this struct implements the `Default` trait, you can also call 
    /// the `default()` method.
    /// 
    /// ```
    /// let scr = scramble::Scr::default();
    /// ``` 
    pub fn new() -> Self {
        let (w, h) = get_dimensions();
        Self {
            _buffer: vec![vec![ColoredChar::new(' ', "".to_string()); w].clone(); h],
            _current: vec![vec![ColoredChar::new('*', "".to_string()); w].clone(); h],
            _w: w, _h: h,
        }
    }

    /// Updates the terminal window.
    /// 
    /// This method compares the `_buffer` Vec and `_current` Vec to 
    /// determine which positions to update in order to increase efficiency.
    /// 
    /// ## Returns
    /// `()`
    /// 
    /// ## Example
    /// ```
    /// let mut scr = scramble::Scr::new();
    /// let mut x: usize = 0;
    /// scr.update();
    /// ```
    pub fn update(&mut self) {
        for rows in 0..self._buffer.len() {
            for chars in 0..self._buffer[rows].len() {
                if self._current[rows][chars] != self._buffer[rows][chars] {
                    print!("{}", self._buffer[rows][chars].prefix());
                    print!("\x1b[{};{}H{}", rows + 1, chars + 1, self._buffer[rows][chars].contents());
                    print!("{}", colors::RESET);
                    self._current[rows][chars] = self._buffer[rows][chars].clone();
                }
            }
        }
        print!("\x1b[{};{}H", self._h - 1, self._w);
        std::io::stdout().lock().flush().expect("Failed to flush stdout!");
    }
    /// Sets text at a starting postion.
    /// 
    /// ## Parameters:
    /// - `x: &mut usize`: The X coordinate of the starting position.
    /// 
    /// The value automatically increments as the `_buffer` Vec is being set and
    /// is restored after the process has finished.
    /// 
    /// - `y: usize`: The Y coordinate of the starting position.
    /// - `msg: &ColoredText`: An object containing your msg and desired color.
    /// 
    /// ## Returns
    /// The `Scr` object itself. This comes handy if you want to chain multiple 
    /// `.set_text()`s together or append an `.update()` after.
    /// 
    /// ## Example
    /// ```
    /// let mut scr = scramble::Scr::new();
    /// let mut x: usize = 0;
    /// scr.set_text(&mut x, 0, &scramble::ColoredText::new_plain("hello, world!".to_string()));
    /// scr.set_text(&mut x, 1, &scramble::ColoredText::new_plain("this is the second line".to_string()));
    /// scr.update();
    /// ```
    /// 
    /// Or, even better, use chaining:
    /// 
    /// ```
    /// let mut scr = scramble::Scr::new();
    /// let mut x: usize = 0;
    /// scr.set_text(&mut x, 0, &scramble::ColoredText::new_plain("hello, world!".to_string()))
    ///    .set_text(&mut x, 1, &scramble::ColoredText::new_plain("this is the second line".to_string()))
    ///    .update();
    /// ```
    pub fn set_text(&mut self, x: &mut usize, y: usize, msg: &ColoredText) -> &mut Self {
        let x_bak = *x;
        if y >= self._h {
            *x = x_bak;
            return self;
        }

        for i in msg.contents().chars() {
            if *x < self._w {
                self._buffer[y][*x] = ColoredChar::new(i, msg.prefix().to_string());
                *x += 1;
            }
        }
        *x = x_bak;
        self
    }

    /// Clears the `_buffer` Vec.
    /// 
    /// ## Returns
    /// 
    /// The `Scr` object itself. This comes handy if you want to append some 
    /// `.set_text()`s or an `.update()` after this method.
    /// 
    /// ## Example
    /// ```
    /// let mut scr = scramble::Scr::new();
    /// let mut x: usize = 0;
    /// scr.set_text(&mut x, 0, &scramble::ColoredText::new_plain("hello, world!".to_string()));
    /// scr.update();
    /// // pause or wait for user input
    /// scr.clear();
    /// scr.update();
    /// ```
    /// 
    /// Or, even better, use chaining:
    /// 
    /// ```
    /// let mut scr = scramble::Scr::new();
    /// let mut x: usize = 0;
    /// scr.set_text(&mut x, 0, &scramble::ColoredText::new_plain("hello, world!".to_string()))
    ///    .update();
    /// // pause or wait for user input
    /// scr.clear()
    ///    .update();
    /// ```
    pub fn clear(&mut self) -> &mut Self {
        for rows in 0..self._buffer.len() {
            for chars in 0..self._buffer[rows].len() {
                self._buffer[rows][chars] = ColoredChar::new(' ', colors::RESET.to_string());
            }
        }
        self
    }

}