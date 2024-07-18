mod scr;

#[cfg(test)]
mod tests {
    use super::*;
    use scr::colored_text::colors;
    use scr::colored_text::ColoredText;
    #[test]
    fn it_works() {
        let mut scr = scr::Scr::new();
        let mut x: usize = 0;
        let mut tmp = String::new();
        let (w, h) = scr.dimensions();
        scr.set_text(&mut x, 3, &ColoredText::new("hi this is red".to_string(), colors::RED.to_string()))
            .update();
        scr.set_text(&mut x, 4, &ColoredText::new("hi this is blue".to_string(), colors::BLUE.to_string()))
            .update();
        scr.set_text(&mut x, 5, &ColoredText::new("hi this has green bg and black text".to_string(), colors::BG_GREEN.to_string() + &colors::BLACK.to_string()))
            .update();

        for i in 0..30000 {
            scr.set_text(&mut x, 7, &ColoredText::new("look im incrementing in orange -> ".to_string() + &(i+1).to_string(), colors::MAGENTA.to_string()))
            .update();
        }

        scr.set_text(&mut x, 9, &ColoredText::new_plain("done! Press any key to continue".to_string()))
            .update();
        std::io::stdin().read_line(&mut tmp).unwrap();
    }
}
