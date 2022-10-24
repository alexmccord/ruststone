#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    tick: u64,
}

impl Frame {
    pub(crate) fn zero() -> Frame {
        Frame { tick: 0 }
    }

    pub fn new() -> Frame {
        Frame { tick: 1 }
    }

    pub fn tick(&self) -> Frame {
        Frame {
            tick: self.tick + 1,
        }
    }
}
