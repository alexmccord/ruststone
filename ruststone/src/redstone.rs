use std::{cell::RefCell, rc::Rc};

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

#[derive(Clone)]
pub enum Redstone {
    Torch(Rc<RefCell<RedstoneTorch>>),
    Dust(Rc<RefCell<RedstoneDust>>),
}

pub trait RedstoneLogic {
    fn redpower(&self) -> Redpower;
    fn apply(&self);
}

trait RedstoneConnection {
    fn add_outgoing_edge(&mut self, outgoing: &Redstone);
    fn add_incoming_edge(&mut self, incoming: &Redstone);
}

pub trait RedstoneLinking {
    fn link(&self, there: &Redstone);
}

impl RedstoneLogic for Redstone {
    fn redpower(&self) -> Redpower {
        match self {
            Redstone::Torch(torch) => torch.redpower(),
            Redstone::Dust(dust) => dust.redpower(),
        }
    }

    fn apply(&self) {
        match self {
            Redstone::Torch(torch) => torch.apply(),
            Redstone::Dust(dust) => dust.apply(),
        }
    }
}

impl RedstoneLinking for Redstone {
    fn link(&self, there: &Redstone) {
        match self {
            Redstone::Torch(torch) => torch.borrow_mut().add_outgoing_edge(&there),
            Redstone::Dust(dust) => dust.borrow_mut().add_outgoing_edge(&there),
        }

        match there {
            Redstone::Torch(torch) => torch.borrow_mut().add_incoming_edge(self),
            Redstone::Dust(dust) => dust.borrow_mut().add_incoming_edge(self),
        }
    }
}

pub struct RedstoneTorch {
    incoming: Option<Redstone>,
    state: bool,
    outgoing: Vec<Redstone>,
}

impl RedstoneTorch {
    pub fn new() -> Redstone {
        Redstone::Torch(Rc::new(RefCell::new(RedstoneTorch {
            incoming: None,
            state: true,
            outgoing: Vec::new(),
        })))
    }
}

impl RedstoneLogic for Rc<RefCell<RedstoneTorch>> {
    fn redpower(&self) -> Redpower {
        match &self.borrow().incoming {
            Some(incoming) => Redpower::strength(if incoming.redpower().has_power() {
                0
            } else {
                16
            }),
            None => Redpower::strength(16),
        }
    }

    fn apply(&self) {
        if self.redpower().has_power() {
            self.borrow_mut().state = false;
        }

        for outgoing in self.borrow().outgoing.iter() {
            outgoing.apply();
        }
    }
}

impl RedstoneConnection for RedstoneTorch {
    fn add_outgoing_edge(&mut self, outgoing: &Redstone) {
        self.outgoing.push(outgoing.clone());
    }

    fn add_incoming_edge(&mut self, incoming: &Redstone) {
        self.incoming = Some(incoming.clone());
    }
}

pub struct RedstoneDust {
    incoming: Vec<Redstone>,
    strength: u32,
    outgoing: Vec<Redstone>,
}

impl RedstoneDust {
    pub fn new() -> Redstone {
        Redstone::Dust(Rc::new(RefCell::new(RedstoneDust {
            incoming: Vec::new(),
            strength: 0,
            outgoing: Vec::new(),
        })))
    }
}

impl RedstoneLogic for Rc<RefCell<RedstoneDust>> {
    fn redpower(&self) -> Redpower {
        let strengths: Vec<Redpower> = self
            .borrow()
            .incoming
            .iter()
            .map(RedstoneLogic::redpower)
            .collect();

        match strengths.iter().max_by_key(|r| r.strength) {
            Some(max) => Redpower::strength(max.strength.saturating_sub(1)),
            None => Redpower::strength(0),
        }
    }

    fn apply(&self) {
        let redpower = self.redpower();

        if redpower.has_power() {
            self.borrow_mut().strength = redpower.strength;
        }

        for outgoing in self.borrow().outgoing.iter() {
            outgoing.apply();
        }
    }
}

impl RedstoneConnection for RedstoneDust {
    fn add_outgoing_edge(&mut self, outgoing: &Redstone) {
        self.outgoing.push(outgoing.clone());
    }

    fn add_incoming_edge(&mut self, incoming: &Redstone) {
        self.incoming.push(incoming.clone());
    }
}
