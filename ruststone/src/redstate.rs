use std::cell::Cell;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Frame(pub(crate) u64);

impl Frame {
    pub(crate) fn next(&self) -> Frame {
        Frame(self.0 + 1)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Redstate {
    updated_frame: Cell<Option<Frame>>,
    power: Cell<u8>,
    forced: Cell<bool>,
}

impl Redstate {
    pub(crate) fn new() -> Redstate {
        Redstate {
            updated_frame: Cell::new(None),
            power: Cell::new(0),
            forced: Cell::new(false),
        }
    }

    pub fn get_power(&self) -> u8 {
        self.power.get()
    }

    pub fn is_forced(&self) -> bool {
        self.forced.get()
    }

    pub(crate) fn set_power(&self, val: u8, frame: Frame) {
        self.updated_frame.set(Some(frame));
        self.power.set(val);
    }

    pub(crate) fn set_forced(&self, val: bool, frame: Frame) {
        self.updated_frame.set(Some(frame));
        self.forced.set(val);
    }

    pub fn is_on(&self) -> bool {
        self.get_power() > 0 || self.forced.get()
    }

    pub fn is_off(&self) -> bool {
        !self.is_on()
    }

    pub(crate) fn updated_frame(&self) -> Option<Frame> {
        self.updated_frame.get()
    }
}
