use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::Redstate;

pub(crate) type RedstoneRef = Rc<RefCell<Redstone>>;

#[derive(Clone, PartialEq, Eq)]
pub enum Redstone {
    Torch {
        incoming: Option<RedstoneRef>,
        outgoing: Vec<RedstoneRef>,
        redstate: Redstate,
    },
    Dust {
        edges: Vec<RedstoneRef>,
        redstate: Redstate,
    },
    NormalBlock {
        edges: Vec<RedstoneRef>,
        redstate: Redstate,
    },
}

impl Redstone {
    pub fn redstate(&self) -> &Redstate {
        match self {
            Redstone::Torch { redstate, .. } => redstate,
            Redstone::Dust { redstate, .. } => redstate,
            Redstone::NormalBlock { redstate, .. } => redstate,
        }
    }

    pub fn torch() -> RedstoneRef {
        Rc::new(RefCell::new(Redstone::Torch {
            incoming: None,
            outgoing: Vec::new(),
            redstate: Redstate::new(),
        }))
    }

    pub fn dust() -> RedstoneRef {
        Rc::new(RefCell::new(Redstone::Dust {
            edges: Vec::new(),
            redstate: Redstate::new(),
        }))
    }

    pub fn normal_block() -> RedstoneRef {
        Rc::new(RefCell::new(Redstone::NormalBlock {
            edges: Vec::new(),
            redstate: Redstate::new(),
        }))
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
        Redstone::Dust { ref mut edges, .. } => {
            assert!(edges.len() <= 6, "Dust can only connect up to 6 edges");
            edges.push(Rc::clone(there));
        }
        Redstone::NormalBlock { ref mut edges, .. } => {
            assert!(edges.len() <= 6, "Dust can only connect up to 6 edges");
            if let Redstone::NormalBlock { .. } = *there.as_ref().borrow() {
                panic!("NormalBlock cannot accept another NormalBlock as an incoming edge");
            }
            edges.push(Rc::clone(there));
        }
    }

    match *there.borrow_mut() {
        Redstone::Torch {
            ref mut incoming, ..
        } => {
            assert!(Borrow::borrow(incoming).is_none());
            *incoming = Some(Rc::clone(here));
        }
        Redstone::Dust { ref mut edges, .. } => {
            assert!(edges.len() <= 6, "Dust can only connect up to 6 edges");
            edges.push(Rc::clone(here));
        }
        Redstone::NormalBlock { ref mut edges, .. } => {
            assert!(
                edges.len() <= 6,
                "NormalBlock can only connect up to 6 edges"
            );
            if let Redstone::NormalBlock { .. } = *here.as_ref().borrow() {
                panic!("NormalBlock cannot accept another NormalBlock as an incoming edge");
            }
            edges.push(Rc::clone(here));
        }
    }
}
