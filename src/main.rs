mod led;
mod relay;
mod utils;

use crate::led::{neopixel, Rgb};
use crate::relay::Relays;
use crate::utils::{to_string, Result};
use esp_idf_hal::rmt::config::TransmitConfig;
use esp_idf_hal::rmt::TxRmtDriver;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::prelude::*;
use esp_idf_svc::http::server::{EspHttpServer, Method};
use esp_idf_svc::io::Write;
use esp_idf_svc::ipv4::{
    ClientConfiguration as IpClientConfiguration, Configuration as IpConfiguration,
    DHCPClientSettings,
};
use esp_idf_svc::netif::{EspNetif, NetifConfiguration, NetifStack};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{
    AuthMethod, BlockingWifi, ClientConfiguration, Configuration as WifiConfiguration, EspWifi,
    WifiDriver,
};

const HOSTNAME: &str = "green";
const WIFI_SSID: &str = env!("ESP_WIFI_SSID");
const WIFI_PASS: &str = env!("ESP_WIFI_PASS");
const STACK_SIZE: usize = 10_240;
static INDEX_HTML: &str = include_str!("index.html");

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;

    let led = peripherals.pins.gpio38;
    let channel = peripherals.rmt.channel0;
    let config = TransmitConfig::new().clock_divider(1);
    let mut tx = TxRmtDriver::new(channel, led, &config)?;
    neopixel(Rgb::new(25, 25, 25), &mut tx)?;

    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let wifi = WifiDriver::new(peripherals.modem, sys_loop.clone(), Some(nvs))?;
    let mut wifi = configure_wifi(wifi)?;
    connect_wifi(&mut wifi, sys_loop)?;

    let mut server = create_server()?;
    server.fn_handler("/", Method::Get, |req| {
        req.into_ok_response()?.write_all(INDEX_HTML.as_bytes())
    })?;

    let relay = Relays::new(
        peripherals.pins.gpio1,
        peripherals.pins.gpio2,
        peripherals.pins.gpio41,
        peripherals.pins.gpio42,
        peripherals.pins.gpio45,
        peripherals.pins.gpio46,
    )?;
    for channel in &relay {
        channel.borrow_mut().on()?;
        FreeRtos::delay_ms(500);
    }
    FreeRtos::delay_ms(1500);
    for channel in &relay {
        channel.borrow_mut().off()?;
        FreeRtos::delay_ms(500);
    }

    (0..360).cycle().try_for_each(|hue| {
        FreeRtos::delay_ms(10);
        let rgb = Rgb::from_hsv(hue, 100, 20)?;
        neopixel(rgb, &mut tx)
    })
}

fn configure_wifi<'a>(driver: WifiDriver) -> Result<EspWifi> {
    let dhcp_config = IpClientConfiguration::DHCP(DHCPClientSettings {
        hostname: Some(to_string(HOSTNAME)?),
    });
    let netif_config = NetifConfiguration {
        ip_configuration: Some(IpConfiguration::Client(dhcp_config)),
        ..NetifConfiguration::wifi_default_client()
    };
    let mut wifi = EspWifi::wrap_all(
        driver,
        EspNetif::new_with_conf(&netif_config)?,
        #[cfg(esp_idf_esp_wifi_softap_support)]
        EspNetif::new(NetifStack::Ap)?,
    )?;
    let wifi_config = WifiConfiguration::Client(ClientConfiguration {
        ssid: to_string(WIFI_SSID)?,
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: to_string(WIFI_PASS)?,
        channel: None,
        ..Default::default()
    });
    wifi.set_configuration(&wifi_config)?;
    Ok(wifi)
}

fn connect_wifi(wifi: &mut EspWifi, sys_loop: EspSystemEventLoop) -> Result<()> {
    wifi.start()?;
    wifi.connect()?;

    let wifi = BlockingWifi::wrap(wifi, sys_loop)?;
    wifi.wait_netif_up()?;

    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    println!("Connected! Wifi Interface Info: {ip_info:?}");
    Ok(())
}

fn create_server() -> Result<EspHttpServer<'static>> {
    let server_configuration = esp_idf_svc::http::server::Configuration {
        stack_size: STACK_SIZE,
        ..Default::default()
    };

    Ok(EspHttpServer::new(&server_configuration)?)
}
