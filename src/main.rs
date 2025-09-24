use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::sys::EspError;

fn main() -> Result<(), EspError> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    let mut ch1 = PinDriver::output(peripherals.pins.gpio1)?;
    let mut ch2 = PinDriver::output(peripherals.pins.gpio2)?;
    let mut ch3 = PinDriver::output(peripherals.pins.gpio41)?;
    let mut ch4 = PinDriver::output(peripherals.pins.gpio42)?;
    let mut ch5 = PinDriver::output(peripherals.pins.gpio45)?;
    let mut ch6 = PinDriver::output(peripherals.pins.gpio46)?;

    ch1.set_high()?;
    FreeRtos::delay_ms(500);
    ch2.set_high()?;
    FreeRtos::delay_ms(500);
    ch3.set_high()?;
    FreeRtos::delay_ms(500);
    ch4.set_high()?;
    FreeRtos::delay_ms(500);
    ch5.set_high()?;
    FreeRtos::delay_ms(500);
    ch6.set_high()?;
    FreeRtos::delay_ms(2000);
    ch1.set_low()?;
    FreeRtos::delay_ms(500);
    ch2.set_low()?;
    FreeRtos::delay_ms(500);
    ch3.set_low()?;
    FreeRtos::delay_ms(500);
    ch4.set_low()?;
    FreeRtos::delay_ms(500);
    ch5.set_low()?;
    FreeRtos::delay_ms(500);
    ch6.set_low()?;
    Ok(())
}
