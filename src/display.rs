const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

pub trait Display {
    fn english_name(&self) -> String;
    fn definite_article(&self) -> &str {
        "the"
    }
    fn indefinite_article(&self) -> &str {
        let first = self.english_name().chars().nth(0);
        match first {
            None => "a",
            Some(letter) => {
                match VOWELS.contains(&letter) {
                    true => "an",
                    false => "a",
                }
            }
        }
    }
}
