use std::cell::Cell;

use crate::{Frame, Redpower};

pub struct Redstate<T: Default + Copy> {
    last_updated: Cell<Frame>,
    state: Cell<T>,
    redpower: Cell<Option<Redpower>>,
}

impl<T: Default + Copy> Redstate<T> {
    pub fn new() -> Redstate<T> {
        Redstate {
            last_updated: Cell::new(Frame::zero()),
            state: Cell::new(Default::default()),
            redpower: Cell::new(None),
        }
    }

    pub fn get(&self) -> T {
        self.state.get()
    }

    pub fn update_frame(&self, frame: Frame) {
        self.last_updated.set(frame);
    }

    pub fn set(&self, val: T, redpower: Redpower) {
        self.state.set(val);
        self.redpower.set(Some(redpower));
    }

    pub fn dispatchable(&self, frame: Frame) -> bool {
        self.last_updated.get() < frame
    }

    pub fn get_redpower(&self, frame: Frame) -> Option<Redpower> {
        self.update_frame(frame);

        if self.dispatchable(frame) {
            None
        } else {
            self.redpower.get()
        }
    }
}
