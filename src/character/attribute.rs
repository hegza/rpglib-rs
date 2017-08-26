
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Debug)]
pub enum Attribute {
    /// Determines unarmed damage and is directly added to damage with weapons
    Strength,
    /// Determines maximum hitpoints
    Constitution,
    /// Determines maximum stamina
    Endurance,
    /// Determines the amount of available actions
    Swiftness,
}

impl AsRef<Attribute> for Attribute {
    fn as_ref(&self) -> &Self {
        self
    }
}
