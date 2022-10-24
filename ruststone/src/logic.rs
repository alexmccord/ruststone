use std::{cell::RefCell, rc::Rc};

use crate::{Redpower, Redstone, RedstoneDust, RedstoneTorch};

// TODO: Observer pattern? Constraint solving?
pub trait RedstoneLogic {
    fn redpower(&self) -> Redpower;
}

impl RedstoneLogic for Redstone {
    fn redpower(&self) -> Redpower {
        match self {
            Redstone::Torch(torch) => torch.redpower(),
            Redstone::Dust(dust) => dust.redpower(),
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
}
