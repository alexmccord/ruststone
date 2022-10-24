use std::{cell::RefCell, rc::Rc};

use crate::{Redpower, Redstone, RedstoneDust, RedstoneTorch};

pub trait RedstoneLogic {
    fn redpower(&self) -> Redpower;
    fn apply(&self);
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
