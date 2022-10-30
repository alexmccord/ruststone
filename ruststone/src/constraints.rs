use std::{
    cell::Cell,
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use crate::{Redstone, RedstoneRef};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Frame(u64);

impl Frame {
    fn next(&self) -> Frame {
        let Frame(frame) = self;
        Frame(frame + 1)
    }
}

struct Constraint {
    // The next frame this constraint can be dispatched. If `None`, it's dispatchable right away.
    next_dispatch_frame: Cell<Option<Frame>>,
    redstone: RedstoneRef,
}

impl Constraint {
    fn new(redstone: RedstoneRef) -> Rc<Constraint> {
        Rc::new(Constraint {
            next_dispatch_frame: Cell::new(None),
            redstone,
        })
    }

    fn dispatchable(&self, current_frame: Frame) -> bool {
        match self.next_dispatch_frame.get() {
            Some(frame) => frame <= current_frame,
            None => true,
        }
    }

    fn dispatch(&self, current_frame: Frame) {
        assert!(self.dispatchable(current_frame));
    }
}

pub struct ConstraintGraph {
    constraints: Vec<Rc<Constraint>>,
}

impl ConstraintGraph {
    fn new() -> ConstraintGraph {
        ConstraintGraph {
            constraints: Vec::new(),
        }
    }

    pub fn collect(redstone: RedstoneRef) -> ConstraintGraph {
        let mut cgb = ConstraintGraph::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_front(redstone);

        while let Some(current) = queue.pop_front() {
            if visited.contains(&Rc::as_ptr(&current)) {
                continue;
            }

            visited.insert(Rc::as_ptr(&current));
            cgb.make_constraint_for_redstone(current.clone());

            match *current.borrow() {
                Redstone::Torch {
                    ref incoming,
                    ref outgoing,
                } => {
                    if let Some(incoming) = incoming {
                        queue.push_front(incoming.clone())
                    }

                    for redstone_cell in outgoing {
                        queue.push_front(redstone_cell.clone());
                    }
                }
                Redstone::Dust { ref edges } => {
                    for edge in edges {
                        queue.push_front(edge.clone());
                    }
                }
            }
        }

        cgb
    }

    fn make_constraint_for_redstone(&mut self, redstone: RedstoneRef) {
        let c = Constraint::new(redstone.clone());
        self.constraints.push(c.clone());
    }

    pub fn len(&self) -> usize {
        self.constraints.len()
    }

    pub fn solve_constraints(&self) {
        let mut queue = VecDeque::from(self.constraints.clone());
        let frame = Frame(0); // TODO: will be mutable.

        // If we can solve a constraint in the current frame, then we shouldn't
        // advance the frame to the next.
        //
        // Only if we did not solve one single constraint and there exists a constraint
        // that can be dispatched in the future can we advance the frame.
        //
        // We can probably just save ourselves the work and find the earliest frame
        // to immediately skip to, in such a case.

        let mut deferred = VecDeque::new();
        while !queue.is_empty() {
            while let Some(c) = queue.pop_front() {
                if !c.dispatchable(frame) {
                    deferred.push_back(c);
                    continue;
                }

                c.dispatch(frame);
            }

            queue = deferred;
            deferred = VecDeque::new();
        }
    }
}
