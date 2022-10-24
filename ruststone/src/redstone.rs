use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub enum Redstone {
    Torch(Rc<RefCell<RedstoneTorch>>),
    Dust(Rc<RefCell<RedstoneDust>>),
}

impl Redstone {
    pub fn torch() -> Redstone {
        Redstone::Torch(Rc::new(RefCell::new(RedstoneTorch {
            incoming: None,
            state: true,
            outgoing: Vec::new(),
        })))
    }

    pub fn dust() -> Redstone {
        Redstone::Dust(Rc::new(RefCell::new(RedstoneDust {
            incoming: Vec::new(),
            strength: 0,
            outgoing: Vec::new(),
        })))
    }
}

pub struct RedstoneTorch {
    pub(crate) incoming: Option<Redstone>,
    pub(crate) state: bool,
    pub(crate) outgoing: Vec<Redstone>,
}

pub struct RedstoneDust {
    pub(crate) incoming: Vec<Redstone>,
    pub(crate) strength: u32,
    pub(crate) outgoing: Vec<Redstone>,
}
