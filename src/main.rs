mod utils;

use crate::utils::{to_string, Result};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::http::server::{EspHttpServer, Method};
use esp_idf_svc::io::Write;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{AuthMethod, ClientConfiguration, Configuration, EspWifi};

const WIFI_SSID: &str = env!("ESP_WIFI_SSID");
const WIFI_PASS: &str = env!("ESP_WIFI_PASS");
const STACK_SIZE: usize = 10_240;
static INDEX_HTML: &str = include_str!("index.html");

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    let event_loop = EspSystemEventLoop::take()?;
    let partition = EspDefaultNvsPartition::take()?;
    let mut wifi = EspWifi::new(peripherals.modem, event_loop.clone(), Some(partition))?;
    connect_wifi(&mut wifi)?;

    let mut server = create_server()?;
    server.fn_handler("/", Method::Get, |req| {
        req.into_ok_response()?.write_all(INDEX_HTML.as_bytes())
    })?;

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

    core::mem::forget(wifi);
    core::mem::forget(server);

    Ok(())
}

fn connect_wifi(wifi: &mut EspWifi) -> Result<()> {
    let client_config = ClientConfiguration {
        ssid: to_string(WIFI_SSID)?,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: to_string(WIFI_PASS)?,
        channel: None,
        ..Default::default()
    };
    wifi.set_configuration(&Configuration::Client(client_config))?;
    wifi.start()?;
    wifi.connect()?;
    // Wait for connection to happen
    while !wifi.is_connected()? {
        // Get and print connection configuration
        let config = wifi.get_configuration()?;
        println!("Waiting for station {:?}", config);
        FreeRtos::delay_ms(250);
    }
    println!("Connected");
    Ok(())
}

fn create_server() -> Result<EspHttpServer<'static>> {
    let server_configuration = esp_idf_svc::http::server::Configuration {
        stack_size: STACK_SIZE,
        ..Default::default()
    };

    Ok(EspHttpServer::new(&server_configuration)?)
}
