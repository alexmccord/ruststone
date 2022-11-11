use std::{cell::RefCell, rc::Rc};

use crate::Redstate;

pub(crate) type RedstoneRef = Rc<RefCell<Redstone>>;

pub(crate) struct RedstoneTorch {
    pub(crate) name: String,
    pub(crate) incoming: Option<RedstoneRef>,
    pub(crate) outgoing: Vec<RedstoneRef>,
}

pub(crate) struct WeightedEdge {
    pub(crate) weight: u8,
    pub(crate) redstone: RedstoneRef,
}

pub(crate) struct RedstoneDust {
    pub(crate) name: String,
    pub(crate) neighbors: Vec<RedstoneRef>,
    pub(crate) sources: Vec<WeightedEdge>,
}

// Not the Redstone Block! It's just a block like Sandstone.
pub(crate) struct Block {
    pub(crate) name: String,
    pub(crate) incoming: Vec<RedstoneRef>,
    pub(crate) outgoing: Vec<RedstoneRef>,
}

pub struct Redstone {
    pub(crate) redstate: Redstate,
    pub(crate) node: RedstoneNode,
}

pub(crate) enum RedstoneNode {
    Torch(RedstoneTorch),
    Dust(RedstoneDust),
    Block(Block),
}

impl Redstone {
    pub fn name(&self) -> String {
        match &self.node {
            RedstoneNode::Torch(torch) => torch.name.clone(),
            RedstoneNode::Dust(dust) => dust.name.clone(),
            RedstoneNode::Block(block) => block.name.clone(),
        }
    }

    pub fn redstate(&self) -> &Redstate {
        &self.redstate
    }

    pub fn torch(name: &str) -> RedstoneRef {
        Rc::new(RefCell::new(Redstone {
            redstate: Redstate::new(),
            node: RedstoneNode::Torch(RedstoneTorch {
                name: String::from(name),
                incoming: None,
                outgoing: Vec::new(),
            })
        }))
    }

    pub fn dust(name: &str) -> RedstoneRef {
        Rc::new(RefCell::new(Redstone {
            redstate: Redstate::new(),
            node: RedstoneNode::Dust(RedstoneDust {
                name: String::from(name),
                neighbors: Vec::new(),
                sources: Vec::new(),
            })
        }))
    }

    pub fn block(name: &str) -> RedstoneRef {
        Rc::new(RefCell::new(Redstone {
            redstate: Redstate::new(),
            node: RedstoneNode::Block(Block {
                name: String::from(name),
                incoming: Vec::new(),
                outgoing: Vec::new(),
            })
        }))
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
    match here.borrow_mut().node {
        RedstoneNode::Torch(ref mut torch) => {
            assert!(
                torch.outgoing.len() <= 5,
                "Torch can only connect up to 5 edges"
            );
            torch.outgoing.push(Rc::clone(there));
        }
        RedstoneNode::Dust(ref mut dust) => {
            assert!(
                dust.neighbors.len() <= 6,
                "Dust can only connect up to 6 edges"
            );
            dust.neighbors.push(Rc::clone(there));
        }
        RedstoneNode::Block(ref mut block) => {
            assert!(
                block.outgoing.len() <= 6,
                "Dust can only connect up to 6 edges"
            );
            block.outgoing.push(Rc::clone(there));
        }
    }

    match there.borrow_mut().node {
        RedstoneNode::Torch(ref mut torch) => {
            assert!(torch.incoming.is_none());
            torch.incoming = Some(Rc::clone(here));
        }
        RedstoneNode::Dust(ref mut dust) => {
            if here.borrow().is_undirected() {
                assert!(
                    dust.neighbors.len() <= 6,
                    "Dust can only connect up to 6 edges"
                );
                dust.neighbors.push(Rc::clone(here));
            }
        }
        RedstoneNode::Block(ref mut block) => {
            assert!(
                block.incoming.len() <= 6,
                "block can only connect up to 6 edges"
            );
            block.incoming.push(Rc::clone(here));
        }
    }
}

pub fn add_weighted_edge(dust: &RedstoneRef, source: &RedstoneRef, weight: u8) {
    let RedstoneNode::Dust(ref mut dust) = dust.borrow_mut().node else {
        panic!("`dust` must be a Redstone::Dust");
    };

    if let RedstoneNode::Dust(..) = source.borrow().node {
        panic!("`source` cannot be a Redstone::Dust");
    }

    dust.sources.push(WeightedEdge {
        weight,
        redstone: source.clone(),
    });
}
