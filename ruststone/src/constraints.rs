use std::{
    cell::{Cell, RefCell},
    collections::{HashSet, VecDeque},
    rc::Rc,
};

use crate::{Frame, RedstoneNode, RedstoneRef};

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

        match self.redstone.node() {
            RedstoneNode::Torch(torch) => {
                match *torch.incoming.borrow() {
                    Some(ref incoming) => self.redstone.redstate().set_power(
                        if incoming.redstate().is_on() { 0 } else { 16 },
                        current_frame,
                    ),
                    None => self.redstone.redstate().set_power(16, current_frame),
                }

                for out in torch.outgoing.borrow().iter() {
                    extra.push(Constraint::new(out.clone(), Some(self.redstone.clone())))
                }
            }
            RedstoneNode::Dust(dust) => {
                let Some((max, weight)) = dust.sources
                    .borrow()
                    .iter()
                    .map(|e| (e.redstone.redstate().clone(), e.weight))
                    .filter(|(r, _)| r.updated_frame() == Some(current_frame))
                    .max_by_key(|(r, w)| r.get_power().saturating_sub(*w))
                else {
                    return extra;
                };

                self.redstone
                    .redstate()
                    .set_power(max.get_power().saturating_sub(weight), current_frame);

                for neighbor in dust.neighbors.borrow().iter() {
                    if !self.is_created_by(neighbor) {
                        extra.push(Constraint::new(
                            neighbor.clone(),
                            Some(self.redstone.clone()),
                        ))
                    }
                }
            }
            RedstoneNode::Block(block) => {
                let has_power = block.incoming.borrow().iter().any(|r| r.redstate().is_on());
                let is_forced = block
                    .incoming
                    .borrow()
                    .iter()
                    .any(|r| r.redstate().is_forced());

                self.redstone
                    .redstate()
                    .set_forced(has_power, current_frame);
                self.redstone
                    .redstate()
                    .set_power(if is_forced { 16 } else { 0 }, current_frame);

                for out in block.outgoing.borrow().iter() {
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

            match current.node() {
                RedstoneNode::Torch(torch) => {
                    cg.constraints.push(Constraint::new(current.clone(), None));

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

                    for c in extra_constraints {
                        queue.push_front(c);
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
