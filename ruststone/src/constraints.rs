use std::{
    cell::{Cell, RefCell},
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use crate::{Frame, Redstone, RedstoneRef};

struct Constraint {
    // The next frame this constraint can be dispatched. If `None`, it's dispatchable right away.
    next_dispatch_frame: Cell<Option<Frame>>,
    redstone: RedstoneRef,
    created_by: Option<RedstoneRef>,
}

impl Constraint {
    fn new(redstone: RedstoneRef, created_by: Option<RedstoneRef>) -> Rc<Constraint> {
        Rc::new(Constraint {
            next_dispatch_frame: Cell::new(None),
            redstone,
            created_by,
        })
    }

    fn dispatchable(&self, current_frame: Frame) -> bool {
        match self.next_dispatch_frame.get() {
            Some(frame) => frame <= current_frame,
            None => true,
        }
    }

    fn dispatch(&self, current_frame: Frame) -> Vec<Rc<Constraint>> {
        assert!(self.dispatchable(current_frame));
        let mut extra = Vec::new();

        match *self.redstone.borrow() {
            Redstone::Torch {
                ref incoming,
                ref outgoing,
                ref redstate,
                ..
            } => {
                match incoming {
                    Some(incoming) => redstate.set_power(
                        if incoming.borrow().redstate().is_on() {
                            0
                        } else {
                            16
                        },
                        current_frame,
                    ),
                    None => redstate.set_power(16, current_frame),
                }

                for out in outgoing {
                    extra.push(Constraint::new(out.clone(), Some(self.redstone.clone())))
                }
            }
            Redstone::Dust {
                ref edges,
                ref redstate,
                ..
            } => {
                let Some(max) = edges
                    .iter()
                    .map(|r| r.borrow().redstate().clone())
                    .filter(|r| r.updated_frame() == Some(current_frame))
                    .max_by_key(|r| r.get_power())
                else {
                    return extra;
                };

                redstate.set_power(max.get_power().saturating_sub(1), current_frame);

                for edge in edges {
                    if let Redstone::Torch { .. } = *edge.borrow() {
                        continue;
                    }

                    if !self.is_created_by(edge) {
                        extra.push(Constraint::new(edge.clone(), Some(self.redstone.clone())))
                    }
                }
            }
            Redstone::NormalBlock {
                ref incoming,
                ref outgoing,
                ref redstate,
                ..
            } => {
                let has_power = incoming.iter().any(|r| r.borrow().redstate().is_on());
                let is_forced = incoming.iter().any(|r| r.borrow().redstate().is_forced());

                redstate.set_forced(has_power, current_frame);
                redstate.set_power(if is_forced { 16 } else { 0 }, current_frame);

                for out in outgoing {
                    if !self.is_created_by(out) {
                        extra.push(Constraint::new(out.clone(), Some(self.redstone.clone())));
                    }
                }
            }
        }

        extra
    }

    fn is_created_by(&self, by: &RedstoneRef) -> bool {
        match self.created_by {
            Some(ref created_by) => Rc::as_ptr(created_by) == Rc::as_ptr(by),
            None => false,
        }
    }
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
            cg.constraints.push(Constraint::new(current.clone(), None));

            match *current.borrow() {
                Redstone::Torch {
                    ref incoming,
                    ref outgoing,
                    ..
                } => {
                    if let Some(incoming) = incoming {
                        queue.push_back(incoming.clone())
                    }

                    for redstone_cell in outgoing {
                        queue.push_back(redstone_cell.clone());
                    }
                }
                Redstone::Dust { ref edges, .. } => {
                    for edge in edges {
                        queue.push_back(edge.clone());
                    }
                }
                Redstone::NormalBlock {
                    ref incoming,
                    ref outgoing,
                    ..
                } => {
                    for incoming in incoming {
                        queue.push_back(incoming.clone());
                    }

                    for outgoing in outgoing {
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
                    self.push_event(c.redstone.borrow().name() + " was deferred!");
                    deferred.push_back(c);
                    continue;
                }

                let previous_state = c.redstone.borrow().redstate().is_on();
                let extra_constraints = c.dispatch(frame);
                let new_state = c.redstone.borrow().redstate().is_on();

                self.push_event(c.redstone.borrow().name() + " was dispatched!");
                self.push_event(previous_state.to_string() + " to " + &new_state.to_string());
                self.push_event(extra_constraints.len().to_string() + " new constraints queued");

                for c in extra_constraints {
                    queue.push_back(c);
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
