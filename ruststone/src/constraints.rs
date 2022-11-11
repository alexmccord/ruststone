use std::{
    cell::{Cell, RefCell},
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use crate::{Frame, RedstoneNode, RedstoneRef};

pub(crate) struct ConstraintCtxt {
    pub(crate) current_frame: Frame,
    pub(crate) redstone: RedstoneRef,
}

pub(crate) struct Constraint {
    // The next frame this constraint can be dispatched. If `None`, it's dispatchable right away.
    next_dispatch_frame: Cell<Option<Frame>>,
    redstone: RedstoneRef,
}

impl Constraint {
    pub(crate) fn new(redstone: RedstoneRef) -> Rc<Constraint> {
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

    fn dispatch(&self, frame: Frame) -> Vec<Rc<Constraint>> {
        assert!(self.dispatchable(frame));
        self.redstone.dispatch(ConstraintCtxt {
            current_frame: frame,
            redstone: self.redstone.clone(),
        })
    }
}

pub(crate) trait ConstraintDispatch {
    fn dispatch(&self, ctxt: ConstraintCtxt) -> Vec<Rc<Constraint>>;
}

pub struct ConstraintGraph {
    constraints: Vec<Rc<Constraint>>,
    events: RefCell<Vec<String>>,
}

impl ConstraintGraph {
    fn new() -> ConstraintGraph {
        ConstraintGraph {
            constraints: Vec::new(),
            events: RefCell::new(Vec::new()),
        }
    }

    pub fn collect(redstone: RedstoneRef) -> ConstraintGraph {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_front(redstone);

        let mut cg = ConstraintGraph::new();

        while let Some(current) = queue.pop_front() {
            if visited.contains(&Rc::as_ptr(&current)) {
                continue;
            }

            visited.insert(Rc::as_ptr(&current));

            match current.node() {
                RedstoneNode::Torch(torch) => {
                    cg.constraints.push(Constraint::new(current.clone()));

                    if let Some(ref incoming) = *torch.incoming.borrow() {
                        queue.push_back(incoming.clone())
                    }

                    for outgoing in torch.outgoing.borrow().iter() {
                        queue.push_back(outgoing.clone());
                    }
                }
                RedstoneNode::Dust(dust) => {
                    for neighbor in dust.neighbors.borrow().iter() {
                        queue.push_back(neighbor.clone());
                    }

                    for source in dust.sources.borrow().iter() {
                        queue.push_back(source.redstone.clone());
                    }
                }
                RedstoneNode::Block(block) => {
                    for incoming in block.incoming.borrow().iter() {
                        queue.push_back(incoming.clone());
                    }

                    for outgoing in block.outgoing.borrow().iter() {
                        queue.push_back(outgoing.clone());
                    }
                }
            }
        }

        cg
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
                    self.push_event(c.redstone.name() + " was deferred!");
                    deferred.push_back(c);
                    continue;
                }

                let previous_state = c.redstone.redstate().clone();
                let extra_constraints = c.dispatch(frame);
                let new_state = c.redstone.redstate().clone();

                self.push_event(c.redstone.name() + " was dispatched!");
                self.push_event(
                    previous_state.is_on().to_string() + " to " + &new_state.is_on().to_string(),
                );

                if previous_state != new_state {
                    self.push_event(
                        extra_constraints.len().to_string() + " new constraints queued",
                    );

                    for e in extra_constraints {
                        queue.push_front(e);
                    }
                }
            }

            queue = deferred;
            deferred = VecDeque::new();
        }
    }

    fn push_event(&self, event: String) {
        self.events.borrow_mut().push(event)
    }
}
