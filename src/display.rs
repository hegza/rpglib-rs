pub const DEFINITE_ARTICLE: &str = "the";
const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

pub trait Display {
    fn name(&self) -> String;
    fn default_article(&self) -> &str {
        self.indefinite_article()
    }
    fn indefinite_article(&self) -> &str {
        let first = self.name().chars().nth(0);
        match first {
            None => "a",
            Some(letter) => match VOWELS.contains(&letter) {
                true => "an",
                false => "a",
            },
        }
    }
}

pub trait DisplayWeapon: Display {
    fn display_offensive_action_1st(&self) -> String {
        "bash".to_owned()
    }
    fn display_offensive_action_2nd(&self) -> String {
        let first_person = self.display_offensive_action_1st();
        let last = first_person.chars().last();
        match last {
            None => "".to_owned(),
            Some(letter) => match VOWELS.contains(&letter) {
                true => format!("{}s", &first_person),
                false => format!("{}es", &first_person),
            },
        }
    }
}
