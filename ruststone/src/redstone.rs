use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Redpower {
    strength: u32,
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

pub type RedstoneCell = Rc<RefCell<Redstone>>;

pub enum Redstone {
    Torch(RedstoneTorch),
    Dust(RedstoneDust),
}

pub trait RedstoneLogic {
    fn redpower(&self) -> Redpower;
    fn apply(&mut self);
}

pub trait RedstoneConnection {
    fn connect(here: RedstoneCell, there: RedstoneCell);
}

impl RedstoneLogic for RedstoneCell {
    fn redpower(&self) -> Redpower {
        match self.borrow().deref() {
            Redstone::Torch(torch) => torch.redpower(),
            Redstone::Dust(dust) => dust.redpower(),
        }
    }

    fn apply(&mut self) {
        match self.borrow_mut().deref_mut() {
            Redstone::Torch(torch) => torch.apply(),
            Redstone::Dust(dust) => dust.apply(),
        }
    }
}

pub struct RedstoneTorch {
    incoming: Option<RedstoneCell>,
    state: bool,
    outgoing: Vec<RedstoneCell>,
}

impl RedstoneTorch {
    pub fn new() -> RedstoneCell {
        Rc::new(RefCell::new(Redstone::Torch(RedstoneTorch {
            incoming: None,
            state: true,
            outgoing: Vec::new(),
        })))
    }
}

impl RedstoneLogic for RedstoneTorch {
    fn redpower(&self) -> Redpower {
        match &self.incoming {
            Some(incoming) => Redpower::strength(if incoming.redpower().has_power() {
                0
            } else {
                16
            }),
            None => Redpower::strength(16),
        }
    }

    fn apply(&mut self) {
        if self.redpower().has_power() {
            self.state = false;
        }

        for outgoing in self.outgoing.iter_mut() {
            outgoing.apply();
        }
    }
}

pub struct RedstoneDust {
    incoming: Vec<RedstoneCell>,
    strength: u32,
    outgoing: Vec<RedstoneCell>,
}

impl RedstoneDust {
    pub fn new() -> RedstoneCell {
        Rc::new(RefCell::new(Redstone::Dust(RedstoneDust {
            incoming: Vec::new(),
            strength: 0,
            outgoing: Vec::new(),
        })))
    }
}

impl RedstoneLogic for RedstoneDust {
    fn redpower(&self) -> Redpower {
        let strengths: Vec<Redpower> = self.incoming.iter().map(RedstoneLogic::redpower).collect();
        match strengths.iter().max_by_key(|r| r.strength) {
            Some(max) => Redpower::strength(max.strength.saturating_sub(1)),
            None => Redpower::strength(0),
        }
    }

    fn apply(&mut self) {
        let incoming_redpower = self.redpower();
        if incoming_redpower.has_power() {
            self.strength = incoming_redpower.strength;
        }

        for outgoing in self.outgoing.iter_mut() {
            outgoing.apply();
        }
    }
}
