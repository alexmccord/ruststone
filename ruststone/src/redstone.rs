use std::{cell::{RefCell}, rc::Rc};

use crate::Redstate;

pub(crate) type RedstoneRef = Rc<Redstone>;

pub struct RedstoneTorch {
    pub(crate) incoming: RefCell<Option<RedstoneRef>>,
    pub(crate) outgoing: RefCell<Vec<RedstoneRef>>,
}

pub(crate) struct WeightedEdge {
    pub(crate) weight: u8,
    pub(crate) redstone: RedstoneRef,
}

pub struct RedstoneDust {
    pub(crate) neighbors: RefCell<Vec<RedstoneRef>>,
    pub(crate) sources: RefCell<Vec<WeightedEdge>>,
}

// Not the Redstone Block! It's just a block like Sandstone.
pub struct Block {
    pub(crate) incoming: RefCell<Vec<RedstoneRef>>,
    pub(crate) outgoing: RefCell<Vec<RedstoneRef>>,
}

pub struct Redstone {
    name: String,
    redstate: Redstate,
    node: RedstoneNode,
}

pub enum RedstoneNode {
    Torch(RedstoneTorch),
    Dust(RedstoneDust),
    Block(Block),
}

impl Redstone {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn redstate(&self) -> &Redstate {
        &self.redstate
    }

    pub fn node(&self) -> &RedstoneNode {
        &self.node
    }

    pub fn torch(name: &str) -> RedstoneRef {
        Rc::new(Redstone {
            name: String::from(name),
            redstate: Redstate::new(),
            node: RedstoneNode::Torch(RedstoneTorch {
                incoming: RefCell::new(None),
                outgoing: RefCell::new(Vec::new()),
            })
        })
    }

    pub fn dust(name: &str) -> RedstoneRef {
        Rc::new(Redstone {
            name: String::from(name),
            redstate: Redstate::new(),
            node: RedstoneNode::Dust(RedstoneDust {
                neighbors: RefCell::new(Vec::new()),
                sources: RefCell::new(Vec::new()),
            })
        })
    }

    pub fn block(name: &str) -> RedstoneRef {
        Rc::new(Redstone {
            name: String::from(name),
            redstate: Redstate::new(),
            node: RedstoneNode::Block(Block {
                incoming: RefCell::new(Vec::new()),
                outgoing: RefCell::new(Vec::new()),
            })
        })
    }

    fn is_directed(&self) -> bool {
        match self.node {
            RedstoneNode::Torch(..) => true,
            RedstoneNode::Dust(..) => false,
            RedstoneNode::Block(..) => false,
        }
    }

    fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}

pub fn link(here: &RedstoneRef, there: &RedstoneRef) {
    match &here.node {
        RedstoneNode::Torch(torch) => {
            assert!(
                torch.outgoing.borrow().len() <= 5,
                "Torch can only connect up to 5 edges"
            );
            torch.outgoing.borrow_mut().push(Rc::clone(there));
        }
        RedstoneNode::Dust(dust) => {
            assert!(
                dust.neighbors.borrow().len() <= 6,
                "Dust can only connect up to 6 edges"
            );
            dust.neighbors.borrow_mut().push(Rc::clone(there));
        }
        RedstoneNode::Block(block) => {
            assert!(
                block.outgoing.borrow().len() <= 6,
                "Dust can only connect up to 6 edges"
            );
            block.outgoing.borrow_mut().push(Rc::clone(there));
        }
    }

    match &there.node {
        RedstoneNode::Torch(torch) => {
            assert!(torch.incoming.borrow().is_none());
            *torch.incoming.borrow_mut() = Some(Rc::clone(here));
        }
        RedstoneNode::Dust(dust) => {
            if here.is_undirected() {
                assert!(
                    dust.neighbors.borrow().len() <= 6,
                    "Dust can only connect up to 6 edges"
                );
                dust.neighbors.borrow_mut().push(Rc::clone(here));
            }
        }
        RedstoneNode::Block(block) => {
            assert!(
                block.incoming.borrow().len() <= 6,
                "block can only connect up to 6 edges"
            );
            block.incoming.borrow_mut().push(Rc::clone(here));
        }
    }
}

pub fn add_weighted_edge(dust: &RedstoneRef, source: &RedstoneRef, weight: u8) {
    let RedstoneNode::Dust(dust) = &dust.node else {
        panic!("`dust` must be a Redstone::Dust");
    };

    if let RedstoneNode::Dust(..) = source.node {
        panic!("`source` cannot be a Redstone::Dust");
    }

    dust.sources.borrow_mut().push(WeightedEdge {
        weight,
        redstone: source.clone(),
    });
}
