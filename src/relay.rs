use crate::utils::Result;
use esp_idf_svc::hal::gpio::Output;
use esp_idf_svc::hal::gpio::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Channel<'d> {
    ch: PinDriver<'d, AnyOutputPin, Output>,
}

pub struct Relays<'d> {
    pub ch1: Rc<RefCell<Channel<'d>>>,
    pub ch2: Rc<RefCell<Channel<'d>>>,
    pub ch3: Rc<RefCell<Channel<'d>>>,
    pub ch4: Rc<RefCell<Channel<'d>>>,
    pub ch5: Rc<RefCell<Channel<'d>>>,
    pub ch6: Rc<RefCell<Channel<'d>>>,
}

impl<'d> Channel<'d> {
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

pub struct RelayIterator<'d> {
    channels: Vec<Rc<RefCell<Channel<'d>>>>,
}

impl<'d> Relays<'d> {
    pub fn new(
        ch1: impl OutputPin + 'd,
        ch2: impl OutputPin + 'd,
        ch3: impl OutputPin + 'd,
        ch4: impl OutputPin + 'd,
        ch5: impl OutputPin + 'd,
        ch6: impl OutputPin + 'd,
    ) -> Result<Self> {
        Ok(Self {
            ch1: Rc::new(RefCell::new(Channel {
                ch: PinDriver::output(ch1.downgrade_output())?,
            })),
            ch2: Rc::new(RefCell::new(Channel {
                ch: PinDriver::output(ch2.downgrade_output())?,
            })),
            ch3: Rc::new(RefCell::new(Channel {
                ch: PinDriver::output(ch3.downgrade_output())?,
            })),
            ch4: Rc::new(RefCell::new(Channel {
                ch: PinDriver::output(ch4.downgrade_output())?,
            })),
            ch5: Rc::new(RefCell::new(Channel {
                ch: PinDriver::output(ch5.downgrade_output())?,
            })),
            ch6: Rc::new(RefCell::new(Channel {
                ch: PinDriver::output(ch6.downgrade_output())?,
            })),
        })
    }
}

impl<'d> IntoIterator for &Relays<'d> {
    type Item = Rc<RefCell<Channel<'d>>>;
    type IntoIter = RelayIterator<'d>;

    fn into_iter(self) -> Self::IntoIter {
        let channels = vec![
            self.ch6.clone(),
            self.ch5.clone(),
            self.ch4.clone(),
            self.ch3.clone(),
            self.ch2.clone(),
            self.ch1.clone(),
        ];
        RelayIterator { channels }
    }
}

impl<'d> Iterator for RelayIterator<'d> {
    type Item = Rc<RefCell<Channel<'d>>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.channels.pop()
    }
}
