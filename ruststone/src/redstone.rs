use std::{
    cell::{Cell, RefCell},
    fmt::Display,
    rc::Rc,
};

use typed_arena::Arena;

use crate::{
    constraints::{Constraint, ConstraintCtxt, ConstraintDispatch, Frame},
    redstate::Redstate,
};

#[derive(Default)]
pub struct RedstoneArena<'r> {
    arena: Arena<Redstone<'r>>,
}

impl<'r> RedstoneArena<'r> {
    pub fn new() -> RedstoneArena<'r> {
        RedstoneArena {
            arena: Arena::new(),
        }
    }

    pub fn make_torch(&'r self, name: &str) -> &Redstone {
        self.arena.alloc(Redstone {
            name: String::from(name),
            redstate: Redstate::zero(),
            node: RedstoneNode::Torch(RedstoneTorch {
                incoming: Cell::new(None),
                outgoing: RefCell::new(Vec::new()),
            }),
        })
    }

    pub fn make_dust(&'r self, name: &str) -> &Redstone {
        self.arena.alloc(Redstone {
            name: String::from(name),
            redstate: Redstate::zero(),
            node: RedstoneNode::Dust(RedstoneDust {
                neighbors: RefCell::new(Vec::new()),
                sources: RefCell::new(Vec::new()),
            }),
        })
    }

    pub fn make_block(&'r self, name: &str) -> &Redstone {
        self.arena.alloc(Redstone {
            name: String::from(name),
            redstate: Redstate::zero(),
            node: RedstoneNode::Block(Block {
                incoming: RefCell::new(Vec::new()),
                outgoing: RefCell::new(Vec::new()),
            }),
        })
    }

    pub fn make_repeater(&'r self, name: &str, delay: u8) -> &Redstone {
        assert!((1..=4).contains(&delay));
        self.arena.alloc(Redstone {
            name: String::from(name),
            redstate: Redstate::zero(),
            node: RedstoneNode::Repeater(RedstoneRepeater {
                delay: Frame(delay.into()),
                incoming: Cell::new(None),
                outgoing: Cell::new(None),
                neighbors: RefCell::new(Vec::new()),
            }),
        })
    }
}

pub struct RedstoneTorch<'r> {
    pub(crate) incoming: Cell<Option<&'r Redstone<'r>>>,
    pub(crate) outgoing: RefCell<Vec<&'r Redstone<'r>>>,
}

impl<'r> ConstraintDispatch<'r> for RedstoneTorch<'r> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'r>) -> Vec<Rc<Constraint<'r>>> {
        let mut extra = Vec::new();

        match &self.incoming.get() {
            Some(incoming) => ctxt
                .redstone
                .redstate()
                .set_power(if incoming.redstate().is_on() { 0 } else { 16 }),
            None => ctxt.redstone.redstate().set_power(16),
        }

        for out in self.outgoing.borrow().iter() {
            extra.push(Constraint::new(out, ctxt.current_frame));
        }

        extra
    }

    fn dispatch_frame_offset(&self) -> Frame {
        Frame(1)
    }
}

pub struct RedstoneDust<'r> {
    pub(crate) neighbors: RefCell<Vec<&'r Redstone<'r>>>,
    pub(crate) sources: RefCell<Vec<(u8, &'r Redstone<'r>)>>,
}

impl<'r> ConstraintDispatch<'r> for RedstoneDust<'r> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'r>) -> Vec<Rc<Constraint<'r>>> {
        let mut extra = Vec::new();

        let sources = self.sources.borrow();
        let Some((weight, redstate)) = sources
            .iter()
            .map(|(w, r)| (w, r.redstate()))
            .max_by_key(|(&w, r)| r.get_power().saturating_sub(w))
        else {
            return extra;
        };

        ctxt.redstone
            .redstate()
            .set_power(redstate.get_power().saturating_sub(*weight));

        for neighbor in self.neighbors.borrow().iter() {
            extra.push(Constraint::new(neighbor, ctxt.current_frame));
        }

        extra
    }

    fn dispatch_frame_offset(&self) -> Frame {
        Frame(0)
    }
}

// Not the Redstone Block! It's just a block like Sandstone.
pub struct Block<'r> {
    pub(crate) incoming: RefCell<Vec<&'r Redstone<'r>>>,
    pub(crate) outgoing: RefCell<Vec<&'r Redstone<'r>>>,
}

impl<'r> ConstraintDispatch<'r> for Block<'r> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'r>) -> Vec<Rc<Constraint<'r>>> {
        let mut extra = Vec::new();

        let has_power = self.incoming.borrow().iter().any(|r| r.redstate().is_on());
        let is_forced = self
            .incoming
            .borrow()
            .iter()
            .any(|r| r.redstate().is_forced());

        ctxt.redstone.redstate().set_forced(has_power);
        ctxt.redstone
            .redstate()
            .set_power(if is_forced { 16 } else { 0 });

        for out in self.outgoing.borrow().iter() {
            extra.push(Constraint::new(out, ctxt.current_frame));
        }

        extra
    }

    fn dispatch_frame_offset(&self) -> Frame {
        Frame(0)
    }
}

pub struct RedstoneRepeater<'r> {
    pub(crate) delay: Frame,
    pub(crate) incoming: Cell<Option<&'r Redstone<'r>>>,
    pub(crate) outgoing: Cell<Option<&'r Redstone<'r>>>,
    pub(crate) neighbors: RefCell<Vec<&'r Redstone<'r>>>,
}

impl<'r> ConstraintDispatch<'r> for RedstoneRepeater<'r> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'r>) -> Vec<Rc<Constraint<'r>>> {
        let mut extra = Vec::new();

        // If any neighbors are on, we'll need to lock the redstate of this repeater.
        if self.neighbors.borrow().iter().any(|n| n.redstate.is_on()) {
            return extra;
        }

        let Some(incoming) = self.incoming.get() else {
            return extra;
        };

        ctxt.redstone
            .redstate()
            .set_forced(incoming.redstate().is_on());
        ctxt.redstone
            .redstate()
            .set_power(if incoming.redstate().is_on() { 16 } else { 0 });

        if let Some(outgoing) = self.outgoing.get() {
            extra.push(Constraint::new(outgoing, ctxt.current_frame));
        }

        extra
    }

    fn dispatch_frame_offset(&self) -> Frame {
        self.delay
    }
}

pub enum RedstoneNode<'r> {
    Torch(RedstoneTorch<'r>),
    Dust(RedstoneDust<'r>),
    Block(Block<'r>),
    Repeater(RedstoneRepeater<'r>),
}

pub struct Redstone<'r> {
    name: String,
    redstate: Redstate,
    node: RedstoneNode<'r>,
}

impl<'r> Redstone<'r> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn redstate(&self) -> &Redstate {
        &self.redstate
    }

    pub fn node(&self) -> &RedstoneNode<'r> {
        &self.node
    }

    fn is_directed(&self) -> bool {
        match self.node() {
            RedstoneNode::Torch(..) => true,
            RedstoneNode::Dust(..) => false,
            RedstoneNode::Block(..) => false,
            RedstoneNode::Repeater(..) => true,
        }
    }

    fn is_undirected(&self) -> bool {
        !self.is_directed()
    }
}

impl<'r> Display for Redstone<'r> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name().as_str())
    }
}

impl<'r> ConstraintDispatch<'r> for Redstone<'r> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'r>) -> Vec<Rc<Constraint<'r>>> {
        match self.node() {
            RedstoneNode::Torch(torch) => torch.dispatch(ctxt),
            RedstoneNode::Dust(dust) => dust.dispatch(ctxt),
            RedstoneNode::Block(block) => block.dispatch(ctxt),
            RedstoneNode::Repeater(repeater) => repeater.dispatch(ctxt),
        }
    }

    fn dispatch_frame_offset(&self) -> Frame {
        match self.node() {
            RedstoneNode::Torch(torch) => torch.dispatch_frame_offset(),
            RedstoneNode::Dust(dust) => dust.dispatch_frame_offset(),
            RedstoneNode::Block(block) => block.dispatch_frame_offset(),
            RedstoneNode::Repeater(repeater) => repeater.dispatch_frame_offset(),
        }
    }
}

pub fn link<'r>(here: &'r Redstone<'r>, there: &'r Redstone<'r>) {
    match here.node() {
        RedstoneNode::Torch(torch) => {
            assert!(torch.outgoing.borrow().len() <= 5);
            torch.outgoing.borrow_mut().push(there);
        }
        RedstoneNode::Dust(dust) => {
            assert!(dust.neighbors.borrow().len() <= 6);
            dust.neighbors.borrow_mut().push(there);
        }
        RedstoneNode::Block(block) => {
            assert!(block.outgoing.borrow().len() <= 6);
            block.outgoing.borrow_mut().push(there);
        }
        RedstoneNode::Repeater(repeater) => {
            assert!(repeater.outgoing.get().is_none());
            repeater.outgoing.set(Some(there));
        }
    }

    match there.node() {
        RedstoneNode::Torch(torch) => {
            assert!(torch.incoming.get().is_none());
            torch.incoming.set(Some(here));
        }
        RedstoneNode::Dust(dust) => {
            if here.is_undirected() {
                assert!(dust.neighbors.borrow().len() <= 6);
                dust.neighbors.borrow_mut().push(here);
            }
        }
        RedstoneNode::Block(block) => {
            assert!(block.incoming.borrow().len() <= 6);
            block.incoming.borrow_mut().push(here);
        }
        RedstoneNode::Repeater(repeater) => {
            if here.is_undirected() {
                assert!(repeater.incoming.get().is_none());
                repeater.incoming.set(Some(here));
            }
        }
    }
}

pub fn add_weighted_edge<'r>(
    dust: &'r Redstone<'r>,
    source: &'r Redstone<'r>,
    weight: u8,
) {
    let RedstoneNode::Dust(dust) = dust.node() else {
        panic!("`dust` must be a RedstoneDust");
    };

    if let RedstoneNode::Dust(..) = source.node() {
        panic!("`source` cannot be a RedstoneDust");
    }

    dust.sources.borrow_mut().push((weight, source));
}

pub fn lock<'r>(repeater: &'r Redstone<'r>, edge: &'r Redstone<'r>) {
    let RedstoneNode::Repeater(repeater) = repeater.node() else {
        panic!("`repeater` must be a RedstoneRepeater");
    };

    assert!((0..=2).contains(&repeater.neighbors.borrow().len()));
    assert!(matches!(edge.node(), RedstoneNode::Repeater(..))); // TODO: comparator too.

    repeater.neighbors.borrow_mut().push(edge);
}
