use esp_idf_svc::hal::delay::FreeRtos;
use crate::utils::Result;
use esp_idf_svc::hal::gpio::Output;
use esp_idf_svc::hal::gpio::*;
use crate::relay::RelayPin::{Ch1, Ch2, Ch3, Ch4, Ch5, Ch6};
use crate::relay::RelayState::{Close, Open};

pub enum RelayPin {
    Ch1,
    Ch2,
    Ch3,
    Ch4,
    Ch5,
    Ch6,
}

pub enum RelayState {
    Close,
    Open,
}

pub struct Relay<'a> {
    ch1: PinDriver<'a, Gpio1, Output>,
    ch2: PinDriver<'a, Gpio2, Output>,
    ch3: PinDriver<'a, Gpio41, Output>,
    ch4: PinDriver<'a, Gpio42, Output>,
    ch5: PinDriver<'a, Gpio45, Output>,
    ch6: PinDriver<'a, Gpio46, Output>,
}

impl<'a> Relay<'a> {
    pub fn new(
        ch1: PinDriver<'a, Gpio1, Output>,
        ch2: PinDriver<'a, Gpio2, Output>,
        ch3: PinDriver<'a, Gpio41, Output>,
        ch4: PinDriver<'a, Gpio42, Output>,
        ch5: PinDriver<'a, Gpio45, Output>,
        ch6: PinDriver<'a, Gpio46, Output>,
    ) -> Self {
        Self {
            ch1,
            ch2,
            ch3,
            ch4,
            ch5,
            ch6,
        }
    }
    
    pub fn switch(&mut self, relay: RelayPin, state: RelayState) -> Result<()> {
        match state {
            RelayState::Close => {
                match relay {
                    RelayPin::Ch1 => self.ch1.set_high()?,
                    RelayPin::Ch2 => self.ch2.set_high()?,
                    RelayPin::Ch3 => self.ch3.set_high()?,
                    RelayPin::Ch4 => self.ch4.set_high()?,
                    RelayPin::Ch5 => self.ch5.set_high()?,
                    RelayPin::Ch6 => self.ch6.set_high()?,
                }
            },
            RelayState::Open => {
                match relay {
                    RelayPin::Ch1 => self.ch1.set_low()?,
                    RelayPin::Ch2 => self.ch2.set_low()?,
                    RelayPin::Ch3 => self.ch3.set_low()?,
                    RelayPin::Ch4 => self.ch4.set_low()?,
                    RelayPin::Ch5 => self.ch5.set_low()?,
                    RelayPin::Ch6 => self.ch6.set_low()?,
                }
            },
        };
        Ok(())
    }
    
    pub fn sequence(&mut self) -> Result<()> {
        self.switch(Ch1, Close)?;
        FreeRtos::delay_ms(500);
        self.switch(Ch2, Close)?;
        FreeRtos::delay_ms(500);
        self.switch(Ch3, Close)?;
        FreeRtos::delay_ms(500);
        self.switch(Ch4, Close)?;
        FreeRtos::delay_ms(500);
        self.switch(Ch5, Close)?;
        FreeRtos::delay_ms(500);
        self.switch(Ch6, Close)?;
        FreeRtos::delay_ms(2000);
        self.switch(Ch1, Open)?;
        FreeRtos::delay_ms(500);
        self.switch(Ch2, Open)?;
        FreeRtos::delay_ms(500);
        self.switch(Ch3, Open)?;
        FreeRtos::delay_ms(500);
        self.switch(Ch4, Open)?;
        FreeRtos::delay_ms(500);
        self.switch(Ch5, Open)?;
        FreeRtos::delay_ms(500);
        self.switch(Ch6, Open)?;
        Ok(())
    }
}
