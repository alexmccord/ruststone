use std::{cell::RefCell, rc::Rc};

use crate::Redstate;

#[derive(Clone)]
pub enum Redstone {
    Torch(Rc<RefCell<RedstoneTorch>>),
    Dust(Rc<RefCell<RedstoneDust>>),
}

impl Redstone {
    pub fn torch() -> Redstone {
        Redstone::Torch(Rc::new(RefCell::new(RedstoneTorch {
            incoming: None,
            redstate: Redstate::new(),
            outgoing: Vec::new(),
        })))
    }

    pub fn dust() -> Redstone {
        Redstone::Dust(Rc::new(RefCell::new(RedstoneDust {
            incoming: Vec::new(),
            redstate: Redstate::new(),
            outgoing: Vec::new(),
        })))
    }
}

pub struct RedstoneTorch {
    pub(crate) incoming: Option<Redstone>,
    pub(crate) redstate: Redstate<bool>,
    pub(crate) outgoing: Vec<Redstone>,
}

pub struct RedstoneDust {
    pub(crate) incoming: Vec<Redstone>,
    pub(crate) redstate: Redstate<u32>,
    pub(crate) outgoing: Vec<Redstone>,
}
