use std::{cell::RefCell, rc::Rc};

use crate::Redstate;

pub(crate) type RedstoneRef = Rc<RefCell<Redstone>>;

#[derive(Clone, PartialEq, Eq)]
pub enum Redstone {
    Torch {
        name: String,
        incoming: Option<RedstoneRef>,
        outgoing: Vec<RedstoneRef>,
        redstate: Redstate,
    },
    Dust {
        name: String,
        incoming: Vec<RedstoneRef>,
        outgoing: Vec<RedstoneRef>,
        redstate: Redstate,
    },
    NormalBlock {
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
            Redstone::NormalBlock { name, .. } => name.clone(),
        }
    }

    pub fn redstate(&self) -> &Redstate {
        match self {
            Redstone::Torch { redstate, .. } => redstate,
            Redstone::Dust { redstate, .. } => redstate,
            Redstone::NormalBlock { redstate, .. } => redstate,
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
            incoming: Vec::new(),
            outgoing: Vec::new(),
            redstate: Redstate::new(),
        }))
    }

    pub fn normal_block(name: &str) -> RedstoneRef {
        Rc::new(RefCell::new(Redstone::NormalBlock {
            name: String::from(name),
            incoming: Vec::new(),
            outgoing: Vec::new(),
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
        Redstone::Dust {
            ref mut outgoing, ..
        } => {
            assert!(outgoing.len() <= 6, "Dust can only connect up to 6 edges");
            outgoing.push(Rc::clone(there));
        }
        Redstone::NormalBlock {
            ref mut outgoing, ..
        } => {
            assert!(outgoing.len() <= 6, "Dust can only connect up to 6 edges");
            if let Redstone::NormalBlock { .. } = *there.as_ref().borrow() {
                panic!("NormalBlock cannot accept another NormalBlock as an outgoing edge");
            }
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
            ref mut incoming, ..
        } => {
            assert!(incoming.len() <= 6, "Dust can only connect up to 6 edges");
            incoming.push(Rc::clone(here));
        }
        Redstone::NormalBlock {
            ref mut incoming, ..
        } => {
            assert!(
                incoming.len() <= 6,
                "NormalBlock can only connect up to 6 edges"
            );
            if let Redstone::NormalBlock { .. } = *here.as_ref().borrow() {
                panic!("NormalBlock cannot accept another NormalBlock as an incoming edge");
            }
            incoming.push(Rc::clone(here));
        }
    }
}
