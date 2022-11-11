use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{Constraint, ConstraintCtxt, ConstraintDispatch, Frame, Redstate};

pub(crate) type RedstoneRef = Rc<Redstone>;

pub struct RedstoneTorch {
    pub(crate) incoming: Option<RedstoneRef>,
    pub(crate) outgoing: Vec<RedstoneRef>,
}

impl ConstraintDispatch for RedstoneTorch {
    fn dispatch(&self, ctxt: ConstraintCtxt) -> Vec<Rc<Constraint>> {
        let mut extra = Vec::new();

        match &self.incoming {
            Some(incoming) => ctxt
                .redstone
                .redstate()
                .set_power(if incoming.redstate().is_on() { 0 } else { 16 }),
            None => ctxt.redstone.redstate().set_power(16),
        }

        for out in self.outgoing.iter() {
            extra.push(Constraint::new(out.clone(), ctxt.current_frame));
        }

        extra
    }

    fn dispatch_frame_offset(&self) -> Frame {
        Frame(1)
    }
}

pub(crate) struct WeightedEdge {
    pub(crate) weight: u8,
    pub(crate) redstone: RedstoneRef,
}

pub struct RedstoneDust {
    pub(crate) neighbors: Vec<RedstoneRef>,
    pub(crate) sources: Vec<WeightedEdge>,
}

impl ConstraintDispatch for RedstoneDust {
    fn dispatch(&self, ctxt: ConstraintCtxt) -> Vec<Rc<Constraint>> {
        let mut extra = Vec::new();

        let Some((max, weight)) = self.sources
            .iter()
            .map(|e| (e.redstone.redstate().clone(), e.weight))
            .max_by_key(|(r, w)| r.get_power().saturating_sub(*w))
        else {
            return extra;
        };

        ctxt.redstone
            .redstate()
            .set_power(max.get_power().saturating_sub(weight));

        for neighbor in self.neighbors.iter() {
            extra.push(Constraint::new(neighbor.clone(), ctxt.current_frame));
        }

        extra
    }

    fn dispatch_frame_offset(&self) -> Frame {
        Frame(0)
    }
}

// Not the Redstone Block! It's just a block like Sandstone.
pub struct Block {
    pub(crate) incoming: Vec<RedstoneRef>,
    pub(crate) outgoing: Vec<RedstoneRef>,
}

impl ConstraintDispatch for Block {
    fn dispatch(&self, ctxt: ConstraintCtxt) -> Vec<Rc<Constraint>> {
        let mut extra = Vec::new();

        let has_power = self.incoming.iter().any(|r| r.redstate().is_on());
        let is_forced = self.incoming.iter().any(|r| r.redstate().is_forced());

        ctxt.redstone.redstate().set_forced(has_power);
        ctxt.redstone
            .redstate()
            .set_power(if is_forced { 16 } else { 0 });

        for out in self.outgoing.iter() {
            extra.push(Constraint::new(out.clone(), ctxt.current_frame));
        }

        extra
    }

    fn dispatch_frame_offset(&self) -> Frame {
        Frame(0)
    }
}

pub struct RedstoneRepeater {
    pub(crate) delay: Frame,
    pub(crate) incoming: Option<RedstoneRef>,
    pub(crate) outgoing: Option<RedstoneRef>,
    pub(crate) neighbors: Vec<RedstoneRef>,
}

impl ConstraintDispatch for RedstoneRepeater {
    fn dispatch(&self, ctxt: ConstraintCtxt) -> Vec<Rc<Constraint>> {
        let mut extra = Vec::new();

        // If any neighbors are on, we'll need to lock the redstate of this repeater.
        if self.neighbors.iter().any(|n| n.redstate.is_on()) {
            return extra;
        }

        let Some(incoming) = &self.incoming else {
            return extra;
        };

        ctxt.redstone.redstate().set_forced(incoming.redstate().is_on());
        ctxt.redstone.redstate().set_power(if incoming.redstate().is_on() { 16 } else { 0 });

        if let Some(outgoing) = &self.outgoing {
            extra.push(Constraint::new(outgoing.clone(), ctxt.current_frame));
            extra.push(Constraint::new(outgoing.clone(), ctxt.current_frame + self.dispatch_frame_offset()));
        }

        extra
    }

    fn dispatch_frame_offset(&self) -> Frame {
        self.delay
    }
}

pub enum RedstoneNode {
    Torch(RedstoneTorch),
    Dust(RedstoneDust),
    Block(Block),
    Repeater(RedstoneRepeater),
}

pub struct Redstone {
    name: String,
    redstate: Redstate,
    node: RefCell<RedstoneNode>,
}

impl Redstone {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn redstate(&self) -> &Redstate {
        &self.redstate
    }

    pub fn node(&self) -> impl Deref<Target = RedstoneNode> + '_ {
        self.node.borrow()
    }

    pub fn node_mut(&self) -> impl DerefMut<Target = RedstoneNode> + '_ {
        self.node.borrow_mut()
    }

    pub fn torch(name: &str) -> RedstoneRef {
        Rc::new(Redstone {
            name: String::from(name),
            redstate: Redstate::new(),
            node: RefCell::new(RedstoneNode::Torch(RedstoneTorch {
                incoming: None,
                outgoing: Vec::new(),
            })),
        })
    }

    pub fn dust(name: &str) -> RedstoneRef {
        Rc::new(Redstone {
            name: String::from(name),
            redstate: Redstate::new(),
            node: RefCell::new(RedstoneNode::Dust(RedstoneDust {
                neighbors: Vec::new(),
                sources: Vec::new(),
            })),
        })
    }

    pub fn block(name: &str) -> RedstoneRef {
        Rc::new(Redstone {
            name: String::from(name),
            redstate: Redstate::new(),
            node: RefCell::new(RedstoneNode::Block(Block {
                incoming: Vec::new(),
                outgoing: Vec::new(),
            })),
        })
    }

    pub fn repeater(name: &str, delay: u8) -> RedstoneRef {
        assert!((1..=4).contains(&delay));
        Rc::new(Redstone {
            name: String::from(name),
            redstate: Redstate::new(),
            node: RefCell::new(RedstoneNode::Repeater(RedstoneRepeater {
                delay: Frame(delay.into()),
                incoming: None,
                outgoing: None,
                neighbors: Vec::new(),
            })),
        })
    }

    fn is_directed(&self) -> bool {
        match *self.node() {
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

impl ConstraintDispatch for Redstone {
    fn dispatch(&self, ctxt: ConstraintCtxt) -> Vec<Rc<Constraint>> {
        match &*self.node() {
            RedstoneNode::Torch(torch) => torch.dispatch(ctxt),
            RedstoneNode::Dust(dust) => dust.dispatch(ctxt),
            RedstoneNode::Block(block) => block.dispatch(ctxt),
            RedstoneNode::Repeater(repeater) => repeater.dispatch(ctxt),
        }
    }

    fn dispatch_frame_offset(&self) -> Frame {
        match &*self.node() {
            RedstoneNode::Torch(torch) => torch.dispatch_frame_offset(),
            RedstoneNode::Dust(dust) => dust.dispatch_frame_offset(),
            RedstoneNode::Block(block) => block.dispatch_frame_offset(),
            RedstoneNode::Repeater(repeater) => repeater.dispatch_frame_offset(),
        }
    }
}

pub fn link(here: &RedstoneRef, there: &RedstoneRef) {
    match &mut *here.node_mut() {
        RedstoneNode::Torch(torch) => {
            assert!(torch.outgoing.len() <= 5);
            torch.outgoing.push(Rc::clone(there));
        }
        RedstoneNode::Dust(dust) => {
            assert!(dust.neighbors.len() <= 6);
            dust.neighbors.push(Rc::clone(there));
        }
        RedstoneNode::Block(block) => {
            assert!(block.outgoing.len() <= 6);
            block.outgoing.push(Rc::clone(there));
        }
        RedstoneNode::Repeater(repeater) => {
            assert!(repeater.outgoing.is_none());
            repeater.outgoing = Some(there.clone());
        }
    }

    match &mut *there.node_mut() {
        RedstoneNode::Torch(torch) => {
            assert!(torch.incoming.is_none());
            torch.incoming = Some(Rc::clone(here));
        }
        RedstoneNode::Dust(dust) => {
            if here.is_undirected() {
                assert!(dust.neighbors.len() <= 6);
                dust.neighbors.push(Rc::clone(here));
            }
        }
        RedstoneNode::Block(block) => {
            assert!(block.incoming.len() <= 6);
            block.incoming.push(Rc::clone(here));
        }
        RedstoneNode::Repeater(repeater) => {
            if here.is_undirected() {
                assert!(repeater.incoming.is_none());
                repeater.incoming = Some(here.clone());
            }
        }
    }
}

pub fn add_weighted_edge(dust: &RedstoneRef, source: &RedstoneRef, weight: u8) {
    let RedstoneNode::Dust(ref mut dust) = *dust.node_mut() else {
        panic!("`dust` must be a RedstoneDust");
    };

    if let RedstoneNode::Dust(..) = *source.node() {
        panic!("`source` cannot be a RedstoneDust");
    }

    dust.sources.push(WeightedEdge {
        weight,
        redstone: source.clone(),
    });
}

pub fn lock(repeater: &RedstoneRef, edge: &RedstoneRef) {
    let RedstoneNode::Repeater(ref mut repeater) = *repeater.node_mut() else {
        panic!("`repeater` must be a RedstoneRepeater");
    };

    assert!((0..=2).contains(&repeater.neighbors.len()));
    assert!(matches!(*edge.node(), RedstoneNode::Repeater(..))); // TODO: comparator too.

    repeater.neighbors.push(edge.clone());
}
