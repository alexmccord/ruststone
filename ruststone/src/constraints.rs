use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    ops::Add,
    rc::Rc,
};

use crate::{Redstone, RedstoneNode};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Frame(pub(crate) u64);

impl Add for Frame {
    type Output = Frame;

    fn add(self, rhs: Frame) -> Frame {
        Frame(self.0 + rhs.0)
    }
}

pub(crate) struct ConstraintCtxt<'r> {
    pub(crate) current_frame: Frame,
    pub(crate) redstone: &'r Redstone<'r>,
}

pub(crate) struct Constraint<'r> {
    // The next frame this constraint can be dispatched.
    next_dispatch_frame: Frame,
    redstone: &'r Redstone<'r>,
}

impl<'r> Constraint<'r> {
    pub(crate) fn new(
        redstone: &'r Redstone<'r>,
        next_dispatch_frame: Frame,
    ) -> Rc<Constraint> {
        Rc::new(Constraint {
            next_dispatch_frame,
            redstone,
        })
    }

    fn dispatchable(&self, current_frame: Frame) -> bool {
        (self.next_dispatch_frame + self.redstone.dispatch_frame_offset()) <= current_frame
    }

    fn dispatch(&self, frame: Frame) -> Vec<Rc<Constraint<'r>>> {
        assert!(self.dispatchable(frame));
        self.redstone.dispatch(ConstraintCtxt {
            current_frame: frame,
            redstone: self.redstone,
        })
    }
}

pub(crate) trait ConstraintDispatch<'r> {
    fn dispatch(&self, ctxt: ConstraintCtxt<'r>) -> Vec<Rc<Constraint<'r>>>;

    fn dispatch_frame_offset(&self) -> Frame;
}

struct ConstraintSolvingEvent<'r>(String, &'r RefCell<Vec<String>>);

impl<'r> ConstraintSolvingEvent<'r> {
    fn new(vec: &'r RefCell<Vec<String>>) -> ConstraintSolvingEvent {
        ConstraintSolvingEvent(String::new(), vec)
    }

    fn write<T: ToString>(self, str: T) -> ConstraintSolvingEvent<'r> {
        if self.0.is_empty() {
            ConstraintSolvingEvent(str.to_string(), self.1)
        } else {
            ConstraintSolvingEvent(self.0 + " " + &str.to_string(), self.1)
        }
    }

    fn push(self) {
        self.1.borrow_mut().push(self.0)
    }
}

pub struct ConstraintGraph<'r> {
    constraints: Vec<Rc<Constraint<'r>>>,
    events: RefCell<Vec<String>>,
}

impl<'r> ConstraintGraph<'r> {
    fn new() -> ConstraintGraph<'r> {
        ConstraintGraph {
            constraints: Vec::new(),
            events: RefCell::new(Vec::new()),
        }
    }

    pub fn collect(redstone: &'r Redstone<'r>) -> ConstraintGraph {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_front(redstone);

        let mut cg = ConstraintGraph::new();

        while let Some(current) = queue.pop_front() {
            if visited.contains(&(current as *const Redstone<'r>)) {
                continue;
            }

            visited.insert(current as *const Redstone);

            match current.node() {
                RedstoneNode::Torch(torch) => {
                    cg.constraints.push(Constraint::new(current, Frame(0)));

                    if let Some(incoming) = torch.incoming.get() {
                        queue.push_back(incoming);
                    }

                    for outgoing in torch.outgoing.borrow().iter() {
                        queue.push_back(outgoing);
                    }
                }
                RedstoneNode::Dust(dust) => {
                    for neighbor in dust.neighbors.borrow().iter() {
                        queue.push_back(neighbor);
                    }

                    for (_, source) in dust.sources.borrow().iter() {
                        queue.push_back(source);
                    }
                }
                RedstoneNode::Block(block) => {
                    for incoming in block.incoming.borrow().iter() {
                        queue.push_back(incoming);
                    }

                    for outgoing in block.outgoing.borrow().iter() {
                        queue.push_back(outgoing);
                    }
                }
                RedstoneNode::Repeater(repeater) => {
                    // TODO: This is probably too fragile to rely on for deterministic locking
                    // on this repeater where the neighbors also lock this at the same time.
                    // I'm not sure yet.
                    for neighbor in repeater.neighbors.borrow().iter() {
                        queue.push_back(neighbor);
                    }

                    if let Some(incoming) = repeater.incoming.get() {
                        queue.push_back(incoming);
                    }

                    if let Some(outgoing) = repeater.outgoing.get() {
                        queue.push_back(outgoing);
                    }
                }
            }
        }

        cg
    }

    pub fn solve_constraints(&self) {
        let mut queue = VecDeque::from(self.constraints.clone());
        let mut frame = Frame(0);

        let mut deferred = VecDeque::new();
        while !queue.is_empty() {
            while let Some(c) = queue.pop_front() {
                if !c.dispatchable(frame) {
                    self.new_event()
                        .write(c.redstone)
                        .write("was deferred")
                        .push();
                    deferred.push_back(c);
                    continue;
                }

                let previous_state = c.redstone.redstate().clone();
                let extra_constraints = c.dispatch(frame);
                let new_state = c.redstone.redstate().clone();

                self.new_event()
                    .write(c.redstone)
                    .write("was dispatched, previously")
                    .write(previous_state.is_on())
                    .write("and now")
                    .write(new_state.is_on())
                    .push();

                if previous_state != new_state {
                    self.new_event()
                        .write(extra_constraints.len())
                        .write("new constraints queued")
                        .push();

                    for e in extra_constraints {
                        queue.push_front(e);
                    }
                }
            }

            // When the queue is empty, then we're at a point where deferred may have constraints,
            // in which case we ought to find the earliest dispatchable frame and skip to that.
            //
            // We'll terminate only when both queue and deferred is empty.
            if queue.is_empty() && !deferred.is_empty() {
                let earliest_dispatchable_frame = deferred
                    .iter()
                    .map(|c| c.next_dispatch_frame + c.redstone.dispatch_frame_offset())
                    .min();

                self.new_event()
                    .write("advancing to frame")
                    .write(frame.0)
                    .push();

                frame = earliest_dispatchable_frame.unwrap();
            }

            queue = deferred;
            deferred = VecDeque::new();
        }
    }

    fn new_event(&self) -> ConstraintSolvingEvent {
        ConstraintSolvingEvent::new(&self.events)
    }
}
