use std::io::Write;
pub(crate) mod colored_text;

fn get_dimensions() -> (usize, usize) {
    // returns (width, height)
    term_size::dimensions().expect("Failed to get terminal dimensions!")
}


#[derive(Clone)]
pub struct Scr {
    _buffer: Vec<Vec<colored_text::ColoredChar>>,
    _current: Vec<Vec<colored_text::ColoredChar>>,
    _w: usize, _h: usize,
}

impl Scr {
    pub fn dimensions(&self) -> (usize, usize) {
        (self._w, self._h)
    }

    pub fn new() -> Self {
        let (w, h) = get_dimensions();
        dbg!(w, h);
        let new = Self {
            _buffer: vec![vec![colored_text::ColoredChar::new(' ', "".to_string()); w].clone(); h],
            _current: vec![vec![colored_text::ColoredChar::new('*', "".to_string()); w].clone(); h],
            _w: w, _h: h,
        };
        new
    }

    pub fn update(&mut self) {
        for rows in 0..self._buffer.len() {
            for chars in 0..self._buffer[rows].len() {
                if self._current[rows][chars] != self._buffer[rows][chars] {
                    print!("{}", self._buffer[rows][chars].prefix());
                    print!("\x1b[{};{}H{}", rows + 1, chars + 1, self._buffer[rows][chars].contents());
                    print!("{}", colored_text::colors::RESET);
                    self._current[rows][chars] = self._buffer[rows][chars].clone();
                }
            }
        }
        print!("\x1b[{};{}H", self._h - 1, self._w);
        std::io::stdout().lock().flush().expect("Failed to flush stdout!");
    }

    pub fn set_text(&mut self, x: &mut usize, y: usize, msg: &colored_text::ColoredText) -> &mut Self {
        if y >= self._h {
            *x = 0;
            return self;
        }

        for i in msg.contents().chars() {
            if *x < self._w {
                self._buffer[y][*x] = colored_text::ColoredChar::new(i, msg.prefix().to_string());
                *x += 1;
            }
        }
        *x = 0;
        self
    }

    pub fn clear(&mut self) -> &mut Self {
        for rows in 0..self._buffer.len() {
            for chars in 0..self._buffer[rows].len() {
                self._buffer[rows][chars] = colored_text::ColoredChar::new(' ', colored_text::colors::RESET.to_string());
            }
        }
        self
    }

}