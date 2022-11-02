use std::{
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet, VecDeque},
    rc::Rc,
};

use crate::{Frame, Redstone, RedstoneRef};

struct Constraint {
    // The next frame this constraint can be dispatched. If `None`, it's dispatchable right away.
    next_dispatch_frame: Cell<Option<Frame>>,
    dependencies: RefCell<Vec<Rc<Constraint>>>,
    redstone: RedstoneRef,
    created_by: Option<RedstoneRef>,
}

impl Constraint {
    fn new(redstone: RedstoneRef, created_by: Option<RedstoneRef>) -> Rc<Constraint> {
        Rc::new(Constraint {
            next_dispatch_frame: Cell::new(None),
            dependencies: RefCell::new(Vec::new()),
            redstone,
            created_by,
        })
    }

    fn dispatchable(&self, current_frame: Frame) -> bool {
        let is_current_or_future_frame = match self.next_dispatch_frame.get() {
            Some(frame) => frame <= current_frame,
            None => true,
        };

        let all_dependencies_dispatchable = self
            .dependencies
            .borrow()
            .iter()
            .all(|r| r.dispatchable(current_frame));

        is_current_or_future_frame && all_dependencies_dispatchable
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
                ref incoming,
                ref outgoing,
                ref redstate,
                ..
            } => {
                // A Dust having no edges is disjoint, so it can't
                // possibly have reached this point by now.
                let max = incoming
                    .iter()
                    .map(|r| r.borrow().redstate().clone())
                    .filter(|r| r.updated_frame() == Some(current_frame))
                    .max_by_key(|r| r.get_power())
                    .unwrap();

                redstate.set_power(max.get_power().saturating_sub(1), current_frame);

                for out in outgoing {
                    if !self.is_created_by(out) {
                        extra.push(Constraint::new(out.clone(), Some(self.redstone.clone())))
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

    fn depends_on(&self, constraints: &Vec<Rc<Constraint>>) {
        for c in constraints {
            self.dependencies.borrow_mut().push(c.clone());
        }
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

    fn build_queue(redstone: RedstoneRef) -> VecDeque<RedstoneRef> {
        let mut visited = HashSet::new();
        let mut discovery_queue = VecDeque::new();
        discovery_queue.push_front(redstone);

        // The queue of RedstoneRef to later drain and turn
        // them into a dependency graph of constraints.
        let mut queue = VecDeque::new();

        while let Some(current) = discovery_queue.pop_front() {
            if visited.contains(&Rc::as_ptr(&current)) {
                continue;
            }

            visited.insert(Rc::as_ptr(&current));
            queue.push_back(current.clone());

            match *current.borrow() {
                Redstone::Torch {
                    ref incoming,
                    ref outgoing,
                    ..
                } => {
                    if let Some(incoming) = incoming {
                        discovery_queue.push_front(incoming.clone())
                    }

                    for redstone_cell in outgoing {
                        discovery_queue.push_front(redstone_cell.clone());
                    }
                }
                Redstone::Dust {
                    ref incoming,
                    ref outgoing,
                    ..
                } => {
                    for incoming in incoming {
                        discovery_queue.push_front(incoming.clone());
                    }

                    for outgoing in outgoing {
                        discovery_queue.push_front(outgoing.clone());
                    }
                }
                Redstone::NormalBlock {
                    ref incoming,
                    ref outgoing,
                    ..
                } => {
                    for incoming in incoming {
                        discovery_queue.push_front(incoming.clone());
                    }

                    for outgoing in outgoing {
                        discovery_queue.push_front(outgoing.clone());
                    }
                }
            }
        }

        queue
    }

    pub fn collect(redstone: RedstoneRef) -> ConstraintGraph {
        let mut queue = ConstraintGraph::build_queue(redstone);
        let mut symbols = HashMap::new();

        while let Some(redstone) = queue.pop_front() {
            match *redstone.borrow() {
                Redstone::Torch {
                    incoming: Some(ref incoming),
                    ..
                } => {
                    if let Some(deps) = symbols.get(&Rc::as_ptr(incoming)) {
                        let c = Constraint::new(redstone.clone(), None);
                        c.depends_on(deps);
                        symbols.insert(Rc::as_ptr(&redstone), vec![c]);
                    } else {
                        queue.push_back(redstone.clone());
                    }
                }
                Redstone::Torch { incoming: None, .. } => {
                    let c = Constraint::new(redstone.clone(), None);
                    symbols.insert(Rc::as_ptr(&redstone), vec![c]);
                }
                Redstone::Dust { .. } => {
                    symbols.insert(Rc::as_ptr(&redstone), Vec::new());
                }
                Redstone::NormalBlock { .. } => {
                    symbols.insert(Rc::as_ptr(&redstone), Vec::new());
                }
            }
        }

        let mut cg = ConstraintGraph::new();
        for deps in symbols.values_mut() {
            cg.constraints.append(deps);
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
                self.push_event("it went from ".to_owned() + &previous_state.to_string() + " to " + &new_state.to_string());
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
