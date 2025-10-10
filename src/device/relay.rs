use crate::prelude::*;
use esp_idf_svc::hal::gpio::Output;
use esp_idf_svc::hal::gpio::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Relay<'d> {
    ch: PinDriver<'d, AnyOutputPin, Output>,
}

impl<'d> Relay<'d> {
    pub fn on(&mut self) -> Result<()> {
        self.ch.set_high()?;
        Ok(())
    }

    pub fn off(&mut self) -> Result<()> {
        self.ch.set_low()?;
        Ok(())
    }

    pub fn toggle(&mut self) -> Result<()> {
        self.ch.toggle()?;
        Ok(())
    }
}

pub struct Controller<'d> {
    relays: Vec<Rc<RefCell<Relay<'d>>>>,
}

impl<'d> Controller<'d> {
    pub fn new() -> Self {
        Self { relays: Vec::new() }
    }

    pub fn add_pin(mut self, relay: impl OutputPin + 'd) -> Result<Self> {
        self.relays.push(Rc::new(RefCell::new(Relay {
            ch: PinDriver::output(relay.downgrade_output())?,
        })));
        Ok(self)
    }
}

impl<'d> IntoIterator for &Controller<'d> {
    type Item = Rc<RefCell<Relay<'d>>>;
    type IntoIter = RelayIterator<'d>;

    fn into_iter(self) -> Self::IntoIter {
        RelayIterator {
            relays: self.relays.clone(),
        }
    }
}

pub struct RelayIterator<'d> {
    relays: Vec<Rc<RefCell<Relay<'d>>>>,
}

impl<'d> Iterator for RelayIterator<'d> {
    type Item = Rc<RefCell<Relay<'d>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.relays.pop()
    }
}
