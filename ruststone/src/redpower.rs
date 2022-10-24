#[derive(Debug, PartialEq, Eq)]
pub struct Redpower {
    pub(crate) strength: u32,
}

impl Redpower {
    pub fn new(strength: u32) -> Redpower {
        Redpower { strength }
    }

    pub fn strength(strength: u32) -> Redpower {
        Redpower::new(strength)
    }

    pub fn has_power(&self) -> bool {
        self.strength > 0
    }
}