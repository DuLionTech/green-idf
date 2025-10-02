use crate::prelude::*;
use esp_idf_hal::modem::WifiModemPeripheral;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_svc::eventloop::EspSystemEventLoop;
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
use log::info;
use std::cell::RefCell;

const HOSTNAME: &str = "green";
const WIFI_SSID: &str = env!("ESP_WIFI_SSID");
const WIFI_PASS: &str = env!("ESP_WIFI_PASS");

pub struct Wifi<'d> {
    wifi: RefCell<BlockingWifi<EspWifi<'d>>>,
}

impl<'d> Wifi<'d> {
    pub fn new<M: WifiModemPeripheral>(
        modem: impl Peripheral<P = M> + 'd,
        sys_loop: EspSystemEventLoop,
        nvs: EspDefaultNvsPartition,
    ) -> Result<Self> {
        let wifi_driver = WifiDriver::new(modem, sys_loop.clone(), Some(nvs))?;
        let dhcp_config = IpClientConfiguration::DHCP(DHCPClientSettings {
            hostname: Some(to_string(HOSTNAME)?),
        });
        let netif_config = NetifConfiguration {
            ip_configuration: Some(IpConfiguration::Client(dhcp_config)),
            ..NetifConfiguration::wifi_default_client()
        };
        let mut esp_wifi = EspWifi::wrap_all(
            wifi_driver,
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
        esp_wifi.set_configuration(&wifi_config)?;

        let wifi = RefCell::new(BlockingWifi::wrap(esp_wifi, sys_loop)?);

        Ok(Self { wifi })
    }

    pub fn connect(&self) -> Result<()> {
        let mut wifi = self.wifi.borrow_mut();
        wifi.start()?;
        wifi.connect()?;

        wifi.wait_netif_up()?;

        let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
        info!("Connected! Wifi Interface Info: {ip_info:?}");
        Ok(())
    }
}
