use std::cell::Cell;

#[derive(Clone, PartialEq, Eq)]
pub struct Redstate {
    power: Cell<u8>,
    forced: Cell<bool>,
}

impl Redstate {
    pub(crate) fn zero() -> Redstate {
        Redstate::new(0)
    }

    pub(crate) fn new(power: u8) -> Redstate {
        Redstate {
            power: Cell::new(power),
            forced: Cell::new(false),
        }
    }

    pub fn get_power(&self) -> u8 {
        self.power.get()
    }

    pub fn is_forced(&self) -> bool {
        self.forced.get()
    }

    pub(crate) fn set_power(&self, val: u8) {
        self.power.set(val);
    }

    pub(crate) fn set_forced(&self, val: bool) {
        self.forced.set(val);
    }

    pub fn is_on(&self) -> bool {
        self.get_power() > 0 || self.forced.get()
    }

    pub fn is_off(&self) -> bool {
        !self.is_on()
    }
}
