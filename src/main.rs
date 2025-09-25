mod util;

use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, BlockingWifi, ClientConfiguration, Configuration, EspWifi};
use crate::util::{Result, Text};

const WIFI_SSID: Text = Text::new(env!("ESP_WIFI_SSID"));
const WIFI_PASS: Text = Text::new(env!("ESP_WIFI_PASS"));

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    let event_loop = EspSystemEventLoop::take()?;
    let partition = EspDefaultNvsPartition::take()?;
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, event_loop.clone(), Some(partition))?,
        event_loop,
    );

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

fn connect_wifi(wifi: &mut EspWifi) -> Result<()> {
    let client_config = ClientConfiguration {
        ssid: WIFI_SSID.try_into()?,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: WIFI_PASS.try_into()?,
        channel: None,
        ..Default::default()
    };
    // let wifi_config: Configuration = Configuration::Client()
    Ok(())
}