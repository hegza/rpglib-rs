use std::convert::From;

// TODO: consider adding keyword associations like: prefer, exclude
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Keyword {
    pub id: String,
}

impl From<&'static str> for Keyword {
    fn from(original: &'static str) -> Keyword {
        Keyword {
            id: original.to_owned(),
        }
    }
}

impl From<String> for Keyword {
    fn from(original: String) -> Keyword {
        Keyword { id: original }
    }
}

impl AsRef<Keyword> for Keyword {
    fn as_ref(&self) -> &Keyword {
        &self
    }
}
