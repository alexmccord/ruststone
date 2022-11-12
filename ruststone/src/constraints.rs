use std::{
    cell::RefCell,
    collections::{HashSet, VecDeque},
    ops::Add,
    rc::Rc,
};

use crate::{RedstoneNode, RedstoneRef};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(pub u64);

impl Add for Frame {
    type Output = Frame;

    fn add(self, rhs: Frame) -> Self::Output {
        Frame(self.0 + rhs.0)
    }
}

pub(crate) struct ConstraintCtxt {
    pub(crate) current_frame: Frame,
    pub(crate) redstone: RedstoneRef,
}

pub(crate) struct Constraint {
    // The next frame this constraint can be dispatched.
    next_dispatch_frame: Frame,
    redstone: RedstoneRef,
}

impl Constraint {
    pub(crate) fn new(redstone: RedstoneRef, next_dispatch_frame: Frame) -> Rc<Constraint> {
        Rc::new(Constraint {
            next_dispatch_frame,
            redstone,
        })
    }

    fn dispatchable(&self, current_frame: Frame) -> bool {
        (self.next_dispatch_frame + self.redstone.dispatch_frame_offset()) <= current_frame
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

    fn dispatch_frame_offset(&self) -> Frame;
}

struct ConstraintSolvingEvent<'a>(String, &'a RefCell<Vec<String>>);

impl<'a> ConstraintSolvingEvent<'a> {
    fn new(vec: &'a RefCell<Vec<String>>) -> ConstraintSolvingEvent {
        ConstraintSolvingEvent(String::new(), vec)
    }

    fn write<T: ToString>(self, str: T) -> ConstraintSolvingEvent<'a> {
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

            match *current.node() {
                RedstoneNode::Torch(ref torch) => {
                    cg.constraints
                        .push(Constraint::new(current.clone(), Frame(0)));

                    if let Some(incoming) = &torch.incoming {
                        queue.push_back(incoming.clone())
                    }

                    for outgoing in torch.outgoing.iter() {
                        queue.push_back(outgoing.clone());
                    }
                }
                RedstoneNode::Dust(ref dust) => {
                    for neighbor in dust.neighbors.iter() {
                        queue.push_back(neighbor.clone());
                    }

                    for (_, source) in dust.sources.iter() {
                        queue.push_back(source.clone());
                    }
                }
                RedstoneNode::Block(ref block) => {
                    for incoming in block.incoming.iter() {
                        queue.push_back(incoming.clone());
                    }

                    for outgoing in block.outgoing.iter() {
                        queue.push_back(outgoing.clone());
                    }
                }
                RedstoneNode::Repeater(ref repeater) => {
                    // TODO: This is probably too fragile to rely on for deterministic locking
                    // on this repeater where the neighbors also lock this at the same time.
                    // I'm not sure yet.
                    for neighbor in &repeater.neighbors {
                        queue.push_back(neighbor.clone());
                    }

                    if let Some(incoming) = &repeater.incoming {
                        queue.push_back(incoming.clone());
                    }

                    if let Some(outgoing) = &repeater.outgoing {
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
        let mut frame = Frame(0);

        let mut deferred = VecDeque::new();
        while !queue.is_empty() {
            while let Some(c) = queue.pop_front() {
                if !c.dispatchable(frame) {
                    self.new_event()
                        .write(&c.redstone)
                        .write("was deferred")
                        .push();
                    deferred.push_back(c);
                    continue;
                }

                let previous_state = c.redstone.redstate().clone();
                let extra_constraints = c.dispatch(frame);
                let new_state = c.redstone.redstate().clone();

                self.new_event()
                    .write(&c.redstone)
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
                frame = earliest_dispatchable_frame.unwrap();

                self.new_event()
                    .write("advancing to frame")
                    .write(frame.0)
                    .push();
            }

            queue = deferred;
            deferred = VecDeque::new();
        }
    }

    fn new_event(&self) -> ConstraintSolvingEvent {
        ConstraintSolvingEvent::new(&self.events)
    }
}
