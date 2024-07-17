mod scr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut scr = scr::Scr::new();
        let mut x: usize = 0;
        let mut tmp = String::new();
        let (w, h) = scr.dimensions();
        scr.set_text(&mut x, 0, &"hello".to_string())
                        .set_text(&mut x, 1, &"screen api test".to_string())
                        .set_text(&mut x, 2, &"and this is the third line".to_string())
                        .set_text(&mut x, 3, &"we can chain multiple .set_text() together".to_string())
                        .set_text(&mut x, 23, &("this window has ".to_string() + &w.to_string() + &" cols and ".to_string() + &h.to_string() + &" rows".to_string()))
                        .update();
        for i in 0..30000 {
            tmp = (i + 1).to_string();
            scr.set_text(&mut x, 7, &("look at me! im incrementing -> ".to_string() + &tmp))
                .update();
        }
        scr.set_text(&mut x, 8, &"done!".to_string());
        scr.update();
        std::io::stdin().read_line(&mut tmp).unwrap();
        scr.clear().update();
        std::io::stdin().read_line(&mut tmp).unwrap();
    }
}
