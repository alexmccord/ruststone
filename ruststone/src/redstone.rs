use std::{cell::RefCell, rc::Rc};

use crate::Redstate;

pub(crate) type RedstoneRef = Rc<RefCell<Redstone>>;

pub struct RedstoneTorch {
    pub(crate) name: String,
    pub(crate) incoming: Option<RedstoneRef>,
    pub(crate) outgoing: Vec<RedstoneRef>,
    pub(crate) redstate: Redstate,
}

pub struct WeightedEdge {
    pub(crate) weight: u8,
    pub(crate) redstone: RedstoneRef,
}

pub struct RedstoneDust {
    pub(crate) name: String,
    pub(crate) neighbors: Vec<RedstoneRef>,
    pub(crate) sources: Vec<WeightedEdge>,
    pub(crate) redstate: Redstate,
}

// Not the Redstone Block! It's just a block like Sandstone.
pub struct Block {
    pub(crate) name: String,
    pub(crate) incoming: Vec<RedstoneRef>,
    pub(crate) outgoing: Vec<RedstoneRef>,
    pub(crate) redstate: Redstate,
}

pub enum Redstone {
    Torch(RedstoneTorch),
    Dust(RedstoneDust),
    Block(Block),
}

impl Redstone {
    pub fn name(&self) -> String {
        match self {
            Redstone::Torch(torch) => torch.name.clone(),
            Redstone::Dust(dust) => dust.name.clone(),
            Redstone::Block(block) => block.name.clone(),
        }
    }

    pub fn redstate(&self) -> &Redstate {
        match self {
            Redstone::Torch(torch) => &torch.redstate,
            Redstone::Dust(dust) => &dust.redstate,
            Redstone::Block(block) => &block.redstate,
        }
    }

    pub fn torch(name: &str) -> RedstoneRef {
        Rc::new(RefCell::new(Redstone::Torch(RedstoneTorch {
            name: String::from(name),
            incoming: None,
            outgoing: Vec::new(),
            redstate: Redstate::new(),
        })))
    }

    pub fn dust(name: &str) -> RedstoneRef {
        Rc::new(RefCell::new(Redstone::Dust(RedstoneDust {
            name: String::from(name),
            neighbors: Vec::new(),
            sources: Vec::new(),
            redstate: Redstate::new(),
        })))
    }

    pub fn block(name: &str) -> RedstoneRef {
        Rc::new(RefCell::new(Redstone::Block(Block {
            name: String::from(name),
            incoming: Vec::new(),
            outgoing: Vec::new(),
            redstate: Redstate::new(),
        })))
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
        Redstone::Torch(ref mut torch) => {
            assert!(
                torch.outgoing.len() <= 5,
                "Torch can only connect up to 5 edges"
            );
            torch.outgoing.push(Rc::clone(there));
        }
        Redstone::Dust(ref mut dust) => {
            assert!(
                dust.neighbors.len() <= 6,
                "Dust can only connect up to 6 edges"
            );
            dust.neighbors.push(Rc::clone(there));
        }
        Redstone::Block(ref mut block) => {
            assert!(
                block.outgoing.len() <= 6,
                "Dust can only connect up to 6 edges"
            );
            block.outgoing.push(Rc::clone(there));
        }
    }

    match *there.borrow_mut() {
        Redstone::Torch(ref mut torch) => {
            assert!(torch.incoming.is_none());
            torch.incoming = Some(Rc::clone(here));
        }
        Redstone::Dust(ref mut dust) => {
            if here.borrow().is_undirected() {
                assert!(
                    dust.neighbors.len() <= 6,
                    "Dust can only connect up to 6 edges"
                );
                dust.neighbors.push(Rc::clone(here));
            }
        }
        Redstone::Block(ref mut block) => {
            assert!(
                block.incoming.len() <= 6,
                "block can only connect up to 6 edges"
            );
            block.incoming.push(Rc::clone(here));
        }
    }
}

pub fn add_weighted_edge(dust: &RedstoneRef, source: &RedstoneRef, weight: u8) {
    let Redstone::Dust(ref mut dust) = *dust.borrow_mut() else {
        panic!("`dust` must be a Redstone::Dust");
    };

    if let Redstone::Dust { .. } = *source.borrow() {
        panic!("`source` cannot be a Redstone::Dust");
    }

    dust.sources.push(WeightedEdge {
        weight,
        redstone: source.clone(),
    });
}
