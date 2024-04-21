use std::{cell::RefCell, collections::VecDeque, ops::Add, rc::Rc};

use crate::{Redstone, RedstoneNode};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Frame(pub(crate) u64);

impl Add for Frame {
    type Output = Frame;

    fn add(self, rhs: Frame) -> Frame {
        Frame(self.0 + rhs.0)
    }
}

pub(crate) struct RedstoneEvent<'r> {
    pub(crate) current_frame: Frame,
    pub(crate) redstone: &'r Redstone<'r>,
}

pub(crate) struct RedstoneDispatchCtxt<'r> {
    next_dispatch_frame: Frame,
    redstone: &'r Redstone<'r>,
}

impl<'r> RedstoneDispatchCtxt<'r> {
    pub(crate) fn new(redstone: &'r Redstone<'r>, next_dispatch_frame: Frame) -> Rc<RedstoneDispatchCtxt> {
        Rc::new(RedstoneDispatchCtxt {
            next_dispatch_frame,
            redstone,
        })
    }

    fn dispatchable(&self, current_frame: Frame) -> bool {
        (self.next_dispatch_frame + self.redstone.dispatch_frame_offset()) <= current_frame
    }

    fn dispatch(&self, frame: Frame) -> Vec<Rc<RedstoneDispatchCtxt<'r>>> {
        assert!(self.dispatchable(frame));
        self.redstone.dispatch(RedstoneEvent {
            current_frame: frame,
            redstone: self.redstone,
        })
    }
}

pub(crate) trait RedstoneDispatch<'r> {
    fn dispatch(&self, event: RedstoneEvent<'r>) -> Vec<Rc<RedstoneDispatchCtxt<'r>>>;

    fn dispatch_frame_offset(&self) -> Frame;
}

struct RedstoneDispatchSnapshot<'r>(String, &'r RefCell<Vec<String>>);

impl<'r> RedstoneDispatchSnapshot<'r> {
    fn new(vec: &'r RefCell<Vec<String>>) -> RedstoneDispatchSnapshot {
        RedstoneDispatchSnapshot(String::new(), vec)
    }

    fn write<T: ToString>(self, str: T) -> RedstoneDispatchSnapshot<'r> {
        if self.0.is_empty() {
            RedstoneDispatchSnapshot(str.to_string(), self.1)
        } else {
            RedstoneDispatchSnapshot(self.0 + " " + &str.to_string(), self.1)
        }
    }

    fn push(self) {
        self.1.borrow_mut().push(self.0)
    }
}

pub struct RedstoneGraph<'r> {
    dispatch_ctxts: Vec<Rc<RedstoneDispatchCtxt<'r>>>,
    snapshots: RefCell<Vec<String>>,
}

impl<'r> RedstoneGraph<'r> {
    pub(crate) fn new() -> RedstoneGraph<'r> {
        RedstoneGraph {
            dispatch_ctxts: Vec::new(),
            snapshots: RefCell::new(Vec::new()),
        }
    }

    pub fn collect(redstone: &'r Redstone<'r>) -> RedstoneGraph {
        let mut rg = RedstoneGraph::new();

        for redstone in redstone.into_iter() {
            if let RedstoneNode::Torch(..) = redstone.node() {
                rg.dispatch_ctxts.push(RedstoneDispatchCtxt::new(redstone, Frame(0)));
            }
        }

        rg
    }

    pub fn run(&self) {
        let mut queue = VecDeque::from(self.dispatch_ctxts.clone());
        let mut frame = Frame(0);

        let mut deferred = VecDeque::new();
        while !queue.is_empty() {
            while let Some(c) = queue.pop_front() {
                if !c.dispatchable(frame) {
                    self.new_snapshot()
                        .write(c.redstone)
                        .write("was deferred")
                        .push();

                    deferred.push_back(c);
                    continue;
                }

                let previous_state = c.redstone.redstate().clone();
                let consequents = c.dispatch(frame);
                let new_state = c.redstone.redstate().clone();

                self.new_snapshot()
                    .write(c.redstone)
                    .write("was dispatched, previously")
                    .write(previous_state.is_on())
                    .write("and now")
                    .write(new_state.is_on())
                    .push();

                if previous_state != new_state {
                    self.new_snapshot()
                        .write(consequents.len())
                        .write("new consequents queued")
                        .push();

                    for consequent in consequents {
                        queue.push_front(consequent);
                    }
                }
            }

            // The queue might be empty, but we might've deferred some of the dispatchables.
            // In that case, we ought to find the earliest dispatchable frame and go there.
            // Terminate only when both queue and deferred is empty.
            if queue.is_empty() && !deferred.is_empty() {
                let earliest_dispatchable_frame = deferred
                    .iter()
                    .map(|c| c.next_dispatch_frame + c.redstone.dispatch_frame_offset())
                    .min();

                self.new_snapshot()
                    .write("advancing to frame")
                    .write(frame.0)
                    .push();

                frame = earliest_dispatchable_frame.unwrap();
            }

            queue = deferred;
            deferred = VecDeque::new();
        }
    }

    fn new_snapshot(&self) -> RedstoneDispatchSnapshot {
        RedstoneDispatchSnapshot::new(&self.snapshots)
    }
}
