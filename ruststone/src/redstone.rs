struct Redpower {
    strength: u32,
    forced: bool,
}

impl Redpower {
    fn new(strength: u32, forced: bool) -> Redpower {
        Redpower { strength, forced }
    }

    fn strength(strength: u32) -> Redpower {
        Redpower::new(strength, false)
    }

    fn forced(forced: bool) -> Redpower {
        Redpower::new(0, forced)
    }
}

trait RedstoneLogic {
    fn update(&mut self, incoming: Redpower) -> Redpower;
}

struct RedstoneTorch {
    state: bool,
}

impl RedstoneLogic for RedstoneTorch {
    fn update(&mut self, incoming: Redpower) -> Redpower {
        self.state = incoming.strength == 0;
        Redpower::strength(if self.state { 16 } else { 0 })
    }
}

struct RedstoneDust {
    strength: u32,
}

impl RedstoneLogic for RedstoneDust {
    fn update(&mut self, incoming: Redpower) -> Redpower {
        self.strength = incoming.strength.saturating_sub(1);
        Redpower::strength(self.strength)
    }
}

struct Block {
    forced: bool,
}

impl RedstoneLogic for Block {
    fn update(&mut self, incoming: Redpower) -> Redpower {
        self.forced = incoming.strength > 0;
        Redpower::forced(self.forced)
    }
}

struct RedstoneRepeater {
    // delay: u32,
}

impl RedstoneLogic for RedstoneRepeater {
    fn update(&mut self, incoming: Redpower) -> Redpower {
        Redpower::strength(if incoming.strength > 0 || incoming.forced {
            16
        } else {
            0
        })
    }
}
