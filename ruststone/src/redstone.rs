use std::{cell::RefCell, rc::Rc};

use crate::Redstate;

pub(crate) type RedstoneRef = Rc<RefCell<Redstone>>;

pub struct WeightedEdge {
    pub(crate) weight: u8,
    pub(crate) redstone: RedstoneRef,
}

pub enum Redstone {
    Torch {
        name: String,
        incoming: Option<RedstoneRef>,
        outgoing: Vec<RedstoneRef>,
        redstate: Redstate,
    },
    Dust {
        name: String,
        neighbors: Vec<RedstoneRef>,
        sources: Vec<WeightedEdge>,
        redstate: Redstate,
    },
    Block {
        name: String,
        incoming: Vec<RedstoneRef>,
        outgoing: Vec<RedstoneRef>,
        redstate: Redstate,
    },
}

impl Redstone {
    pub fn name(&self) -> String {
        match self {
            Redstone::Dust { name, .. } => name.clone(),
            Redstone::Torch { name, .. } => name.clone(),
            Redstone::Block { name, .. } => name.clone(),
        }
    }

    pub fn redstate(&self) -> &Redstate {
        match self {
            Redstone::Torch { redstate, .. } => redstate,
            Redstone::Dust { redstate, .. } => redstate,
            Redstone::Block { redstate, .. } => redstate,
        }
    }

    pub fn torch(name: &str) -> RedstoneRef {
        Rc::new(RefCell::new(Redstone::Torch {
            name: String::from(name),
            incoming: None,
            outgoing: Vec::new(),
            redstate: Redstate::new(),
        }))
    }

    pub fn dust(name: &str) -> RedstoneRef {
        Rc::new(RefCell::new(Redstone::Dust {
            name: String::from(name),
            neighbors: Vec::new(),
            sources: Vec::new(),
            redstate: Redstate::new(),
        }))
    }

    pub fn block(name: &str) -> RedstoneRef {
        Rc::new(RefCell::new(Redstone::Block {
            name: String::from(name),
            incoming: Vec::new(),
            outgoing: Vec::new(),
            redstate: Redstate::new(),
        }))
    }

    fn is_directed(&self) -> bool {
        match self {
            Redstone::Torch { .. } => true,
            Redstone::Dust { .. } => false,
            Redstone::Block { .. } => false,
        }
    }

    fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}

pub fn link(here: &RedstoneRef, there: &RedstoneRef) {
    match *here.borrow_mut() {
        Redstone::Torch {
            ref mut outgoing, ..
        } => {
            assert!(outgoing.len() <= 5, "Torch can only connect up to 5 edges");
            outgoing.push(Rc::clone(there));
        }
        Redstone::Dust {
            ref mut neighbors, ..
        } => {
            assert!(neighbors.len() <= 6, "Dust can only connect up to 6 edges");
            neighbors.push(Rc::clone(there));
        }
        Redstone::Block {
            ref mut outgoing, ..
        } => {
            assert!(outgoing.len() <= 6, "Dust can only connect up to 6 edges");
            outgoing.push(Rc::clone(there));
        }
    }

    match *there.borrow_mut() {
        Redstone::Torch {
            ref mut incoming, ..
        } => {
            assert!(incoming.is_none());
            *incoming = Some(Rc::clone(here));
        }
        Redstone::Dust {
            ref mut neighbors, ..
        } => {
            if here.borrow().is_undirected() {
                assert!(neighbors.len() <= 6, "Dust can only connect up to 6 edges");
                neighbors.push(Rc::clone(here));
            }
        }
        Redstone::Block {
            ref mut incoming, ..
        } => {
            assert!(
                incoming.len() <= 6,
                "NormalBlock can only connect up to 6 edges"
            );
            incoming.push(Rc::clone(here));
        }
    }
}

pub fn add_weighted_edge(dust: &RedstoneRef, source: &RedstoneRef, weight: u8) {
    let Redstone::Dust { ref mut sources, .. } = *dust.borrow_mut() else {
        panic!("`dust` must be a Redstone::Dust");
    };

    if let Redstone::Dust { .. } = *source.borrow() {
        panic!("`source` cannot be a Redstone::Dust");
    }

    sources.push(WeightedEdge {
        weight,
        redstone: source.clone(),
    });
}
