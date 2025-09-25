mod relay;
mod utils;

use crate::relay::Relay;
use crate::utils::{to_string, Result};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::gpio::PinDriver;
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
    AuthMethod, BlockingWifi, ClientConfiguration, Configuration as WifiConfiguration, EspWifi, WifiDriver,
};
const WIFI_SSID: &str = env!("ESP_WIFI_SSID");
const WIFI_PASS: &str = env!("ESP_WIFI_PASS");
const STACK_SIZE: usize = 10_240;
static INDEX_HTML: &str = include_str!("index.html");

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let wifi = WifiDriver::new(peripherals.modem, sys_loop.clone(), Some(nvs))?;
    let mut wifi = configure_wifi(wifi)?;
    connect_wifi(&mut wifi, sys_loop)?;

    let mut server = create_server()?;
    server.fn_handler("/", Method::Get, |req| {
        req.into_ok_response()?.write_all(INDEX_HTML.as_bytes())
    })?;

    let mut relays = Relay::new(
        PinDriver::output(peripherals.pins.gpio1)?,
        PinDriver::output(peripherals.pins.gpio2)?,
        PinDriver::output(peripherals.pins.gpio41)?,
        PinDriver::output(peripherals.pins.gpio42)?,
        PinDriver::output(peripherals.pins.gpio45)?,
        PinDriver::output(peripherals.pins.gpio46)?,
    );
    relays.sequence()?;

    loop {
        FreeRtos::delay_ms(1000);
    }
}

fn configure_wifi<'a>(wifi: WifiDriver) -> Result<EspWifi> {
    let dhcp_config = IpClientConfiguration::DHCP(
        DHCPClientSettings {
            hostname: Some(to_string("green")?),
        },
    );
    let netif_config = NetifConfiguration {
        ip_configuration: Some(IpConfiguration::Client(dhcp_config)),
        ..NetifConfiguration::wifi_default_client()
    };
    let mut wifi = EspWifi::wrap_all(
        wifi,
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
