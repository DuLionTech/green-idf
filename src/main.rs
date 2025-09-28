mod led;
mod relay;
mod server;
mod utils;
mod wifi;

use crate::led::{NeoPixel, Rgb};
use crate::relay::Relays;
use crate::server::Server;
use crate::utils::Result;
use crate::wifi::Wifi;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::nvs::EspDefaultNvsPartition;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;

    // Configure LED
    let mut pixel = NeoPixel::new(peripherals.rmt.channel0, peripherals.pins.gpio38)?;
    pixel.send(Rgb::new(25, 25, 25))?;

    // Configure WiFi
    let wifi = Wifi::new(
        peripherals.modem,
        EspSystemEventLoop::take()?,
        EspDefaultNvsPartition::take()?,
    )?;
    wifi.connect()?;

    // Configure HTTP server
    let mut server = Server::new()?;
    server.initial_handlers()?;

    // Configure relays
    let relays = Relays::new(
        peripherals.pins.gpio1,
        peripherals.pins.gpio2,
        peripherals.pins.gpio41,
        peripherals.pins.gpio42,
        peripherals.pins.gpio45,
        peripherals.pins.gpio46,
    )?;

    // Blink relays
    for channel in &relays {
        channel.borrow_mut().on()?;
        FreeRtos::delay_ms(500);
    }
    FreeRtos::delay_ms(1500);
    for channel in &relays {
        channel.borrow_mut().off()?;
        FreeRtos::delay_ms(500);
    }

    // HSV color cycle
    (0..360).cycle().try_for_each(|hue| {
        FreeRtos::delay_ms(10);
        let rgb = Rgb::from_hsv(hue, 100, 20)?;
        pixel.send(rgb)
    })
}
