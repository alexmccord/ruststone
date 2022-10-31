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
    state: Cell<u8>,
}

impl Redstate {
    pub(crate) fn new() -> Redstate {
        Redstate {
            updated_frame: Cell::new(None),
            state: Default::default(),
        }
    }

    pub fn get(&self) -> u8 {
        self.state.get()
    }

    pub(crate) fn set(&self, val: u8, frame: Frame) {
        self.updated_frame.set(Some(frame));
        self.state.set(val);
    }

    pub fn is_on(&self) -> bool {
        self.get() > 0
    }

    pub fn is_off(&self) -> bool {
        !self.is_on()
    }

    pub(crate) fn updated_frame(&self) -> Option<Frame> {
        self.updated_frame.get()
    }
}
