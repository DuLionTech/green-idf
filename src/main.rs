mod device;
mod net;
mod prelude;

use crate::prelude::*;
use device::led::{NeoPixel, Rgb};
use device::relay::Controller;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use log::info;
use net::http::Http;
use net::mqtt::Mqtt;
use net::wifi::Wifi;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;

    // Configure LED
    let mut pixel = NeoPixel::new(peripherals.rmt.channel0, peripherals.pins.gpio38)?;
    pixel.update(Rgb::new(25, 25, 25))?;

    // Configure WiFi
    info!("Starting WiFi");
    let wifi = Wifi::new(
        peripherals.modem,
        EspSystemEventLoop::take()?,
        EspDefaultNvsPartition::take()?,
    )?;
    wifi.connect()?;

    // Configure HTTP server
    info!("Starting HTTP server");
    let mut server = Http::new()?;
    server.start()?;

    // Configure MQTT
    info!("Starting MQTT");
    let mut mqtt = Mqtt::new()?;
    mqtt.start()?;

    // Configure relays
    let relays = Controller::new()
        .add_pin(peripherals.pins.gpio1)?
        .add_pin(peripherals.pins.gpio2)?
        .add_pin(peripherals.pins.gpio41)?
        .add_pin(peripherals.pins.gpio42)?
        .add_pin(peripherals.pins.gpio45)?
        .add_pin(peripherals.pins.gpio46)?;

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
        pixel.update(rgb)
    })
}
