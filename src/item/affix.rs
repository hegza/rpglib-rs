use super::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Affix {
    pub effects: Vec<ItemEffect>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prefix {
    pub affix_data: Affix,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Suffix {
    pub affix_data: Affix,
}

impl From<Prefix> for Affix {
    fn from(original: Prefix) -> Self {
        original.affix_data
    }
}

impl From<Suffix> for Affix {
    fn from(original: Suffix) -> Self {
        original.affix_data
    }
}

impl From<Affix> for Prefix {
    fn from(original: Affix) -> Self {
        Prefix { affix_data: original }
    }
}

impl From<Affix> for Suffix {
    fn from(original: Affix) -> Self {
        Suffix { affix_data: original }
    }
}

impl AsRef<Affix> for Affix {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Prefix> for Prefix {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<Suffix> for Suffix {
    fn as_ref(&self) -> &Self {
        self
    }
}
