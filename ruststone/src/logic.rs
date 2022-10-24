use std::{cell::RefCell, rc::Rc};

use crate::{Frame, Redpower, Redstone, RedstoneDust, RedstoneTorch};

pub trait RedstoneLogic {
    fn try_use_computed_redpower(&self, frame: Frame) -> Option<Redpower>;
    fn redpower(&self, frame: Frame) -> Redpower;
    fn apply(&self, frame: Frame);
}

impl RedstoneLogic for Redstone {
    fn try_use_computed_redpower(&self, frame: Frame) -> Option<Redpower> {
        match self {
            Redstone::Torch(torch) => torch.try_use_computed_redpower(frame),
            Redstone::Dust(dust) => dust.try_use_computed_redpower(frame),
        }
    }

    fn redpower(&self, frame: Frame) -> Redpower {
        if let Some(redpower) = self.try_use_computed_redpower(frame) {
            return redpower;
        }

        match self {
            Redstone::Torch(torch) => torch.redpower(frame),
            Redstone::Dust(dust) => dust.redpower(frame),
        }
    }

    fn apply(&self, frame: Frame) {
        assert!(self.try_use_computed_redpower(frame).is_none()); // TODO: is this right?

        match self {
            Redstone::Torch(torch) => torch.apply(frame),
            Redstone::Dust(dust) => dust.apply(frame),
        }
    }
}

impl RedstoneLogic for Rc<RefCell<RedstoneTorch>> {
    fn try_use_computed_redpower(&self, frame: Frame) -> Option<Redpower> {
        self.borrow().redstate.get_redpower(frame)
    }

    fn redpower(&self, frame: Frame) -> Redpower {
        match &self.borrow().incoming {
            Some(incoming) => Redpower::strength(if incoming.redpower(frame).has_power() {
                0
            } else {
                16
            }),
            None => Redpower::strength(16),
        }
    }

    fn apply(&self, frame: Frame) {
        let redpower = self.redpower(frame);
        self.borrow().redstate.set(redpower.has_power(), redpower);

        for outgoing in self.borrow().outgoing.iter() {
            outgoing.apply(frame);
        }
    }
}

impl RedstoneLogic for Rc<RefCell<RedstoneDust>> {
    fn try_use_computed_redpower(&self, frame: Frame) -> Option<Redpower> {
        self.borrow().redstate.get_redpower(frame)
    }

    fn redpower(&self, frame: Frame) -> Redpower {
        let strengths: Vec<Redpower> = self
            .borrow()
            .incoming
            .iter()
            .map(|r| r.redpower(frame))
            .collect();

        match strengths.iter().max_by_key(|r| r.strength) {
            Some(max) => Redpower::strength(max.strength.saturating_sub(1)),
            None => Redpower::strength(0),
        }
    }

    fn apply(&self, frame: Frame) {
        let redpower = self.redpower(frame);
        self.borrow().redstate.set(redpower.strength, redpower);

        for outgoing in self.borrow().outgoing.iter() {
            outgoing.apply(frame);
        }
    }
}
