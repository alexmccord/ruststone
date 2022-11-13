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
pub struct RedstoneArena<'rctx> {
    arena: Arena<Redstone<'rctx>>,
}

impl<'rctx> RedstoneArena<'rctx> {
    pub fn new() -> RedstoneArena<'rctx> {
        RedstoneArena {
            arena: Arena::new(),
        }
    }

    pub fn torch(&'rctx self, name: &str) -> &Redstone {
        self.arena.alloc(Redstone {
            name: String::from(name),
            redstate: Redstate::zero(),
            node: RedstoneNode::Torch(RedstoneTorch {
                incoming: Cell::new(None),
                outgoing: RefCell::new(Vec::new()),
            }),
        })
    }

    pub fn dust(&'rctx self, name: &str) -> &Redstone {
        self.arena.alloc(Redstone {
            name: String::from(name),
            redstate: Redstate::zero(),
            node: RedstoneNode::Dust(RedstoneDust {
                neighbors: RefCell::new(Vec::new()),
                sources: RefCell::new(Vec::new()),
            }),
        })
    }

    pub fn block(&'rctx self, name: &str) -> &Redstone {
        self.arena.alloc(Redstone {
            name: String::from(name),
            redstate: Redstate::zero(),
            node: RedstoneNode::Block(Block {
                incoming: RefCell::new(Vec::new()),
                outgoing: RefCell::new(Vec::new()),
            }),
        })
    }

    pub fn repeater(&'rctx self, name: &str, delay: u8) -> &Redstone {
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

pub struct RedstoneTorch<'rctx> {
    pub(crate) incoming: Cell<Option<&'rctx Redstone<'rctx>>>,
    pub(crate) outgoing: RefCell<Vec<&'rctx Redstone<'rctx>>>,
}

impl<'rctx> ConstraintDispatch<'rctx> for RedstoneTorch<'rctx> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'rctx>) -> Vec<Rc<Constraint<'rctx>>> {
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

pub struct RedstoneDust<'rctx> {
    pub(crate) neighbors: RefCell<Vec<&'rctx Redstone<'rctx>>>,
    pub(crate) sources: RefCell<Vec<(u8, &'rctx Redstone<'rctx>)>>,
}

impl<'rctx> ConstraintDispatch<'rctx> for RedstoneDust<'rctx> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'rctx>) -> Vec<Rc<Constraint<'rctx>>> {
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
pub struct Block<'rctx> {
    pub(crate) incoming: RefCell<Vec<&'rctx Redstone<'rctx>>>,
    pub(crate) outgoing: RefCell<Vec<&'rctx Redstone<'rctx>>>,
}

impl<'rctx> ConstraintDispatch<'rctx> for Block<'rctx> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'rctx>) -> Vec<Rc<Constraint<'rctx>>> {
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

pub struct RedstoneRepeater<'rctx> {
    pub(crate) delay: Frame,
    pub(crate) incoming: Cell<Option<&'rctx Redstone<'rctx>>>,
    pub(crate) outgoing: Cell<Option<&'rctx Redstone<'rctx>>>,
    pub(crate) neighbors: RefCell<Vec<&'rctx Redstone<'rctx>>>,
}

impl<'rctx> ConstraintDispatch<'rctx> for RedstoneRepeater<'rctx> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'rctx>) -> Vec<Rc<Constraint<'rctx>>> {
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

pub enum RedstoneNode<'rctx> {
    Torch(RedstoneTorch<'rctx>),
    Dust(RedstoneDust<'rctx>),
    Block(Block<'rctx>),
    Repeater(RedstoneRepeater<'rctx>),
}

pub struct Redstone<'rctx> {
    name: String,
    redstate: Redstate,
    node: RedstoneNode<'rctx>,
}

impl<'rctx> Redstone<'rctx> {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn redstate(&self) -> &Redstate {
        &self.redstate
    }

    pub fn node(&self) -> &RedstoneNode<'rctx> {
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

impl<'rctx> Display for Redstone<'rctx> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name().as_str())
    }
}

impl<'rctx> ConstraintDispatch<'rctx> for Redstone<'rctx> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'rctx>) -> Vec<Rc<Constraint<'rctx>>> {
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

pub fn link<'rctx>(here: &'rctx Redstone<'rctx>, there: &'rctx Redstone<'rctx>) {
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

pub fn add_weighted_edge<'rctx>(
    dust: &'rctx Redstone<'rctx>,
    source: &'rctx Redstone<'rctx>,
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

pub fn lock<'rctx>(repeater: &'rctx Redstone<'rctx>, edge: &'rctx Redstone<'rctx>) {
    let RedstoneNode::Repeater(repeater) = repeater.node() else {
        panic!("`repeater` must be a RedstoneRepeater");
    };

    assert!((0..=2).contains(&repeater.neighbors.borrow().len()));
    assert!(matches!(edge.node(), RedstoneNode::Repeater(..))); // TODO: comparator too.

    repeater.neighbors.borrow_mut().push(edge);
}
