use std::{cell::RefCell, rc::Rc, ops::{Deref, DerefMut}};

pub struct Redpower {
    strength: u32,
}

impl Redpower {
    fn new(strength: u32) -> Redpower {
        Redpower { strength }
    }

    fn strength(strength: u32) -> Redpower {
        Redpower::new(strength)
    }

    pub fn has_power(&self) -> bool {
        self.strength > 0
    }
}

pub trait RedstoneLogic {
    fn redpower(&self) -> Redpower;
    fn apply(&mut self);
}

pub enum Redstone {
    Torch(RedstoneTorch),
    Dust(RedstoneDust),
}

impl RedstoneLogic for Rc<RefCell<Redstone>> {
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
    incoming: Option<Rc<RefCell<Redstone>>>,
    state: bool,
    outgoing: Vec<Rc<RefCell<Redstone>>>,
}

impl RedstoneTorch {
    pub fn new() -> Rc<RefCell<Redstone>> {
        Rc::new(RefCell::new(Redstone::Torch(RedstoneTorch {
            incoming: None,
            state: true,
            outgoing: Vec::new(),
        })))
    }
}

impl RedstoneLogic for RedstoneTorch {
    fn redpower(&self) -> Redpower {
        match self.incoming.as_ref() {
            Some(incoming) => Redpower::strength(if incoming.redpower().has_power() {
                16
            } else {
                0
            }),
            None => Redpower::strength(16),
        }
    }

    fn apply(&mut self) {
        if self.redpower().has_power() {
            self.state = false;
        }

        for outgoing in self.outgoing.iter() {
            outgoing.clone().apply();
        }
    }
}

pub struct RedstoneDust {
    incoming: Vec<Rc<RefCell<Redstone>>>,
    strength: u32,
    outgoing: Vec<Rc<RefCell<Redstone>>>,
}

impl RedstoneDust {
    pub fn new() -> Rc<RefCell<Redstone>> {
        Rc::new(RefCell::new(Redstone::Dust(RedstoneDust {
            incoming: Vec::new(),
            strength: 0,
            outgoing: Vec::new(),
        })))
    }
}

impl RedstoneLogic for RedstoneDust {
    fn redpower(&self) -> Redpower {
        match self
            .incoming
            .iter()
            .max_by_key(|r| r.redpower().strength)
        {
            Some(max) => Redpower::strength(max.redpower().strength.saturating_sub(1)),
            None => Redpower::strength(0),
        }
    }

    fn apply(&mut self) {
        let incoming_redpower = self.redpower();
        if incoming_redpower.has_power() {
            self.strength = incoming_redpower.strength;
        }

        for outgoing in self.outgoing.iter() {
            outgoing.clone().apply();
        }
    }
}
