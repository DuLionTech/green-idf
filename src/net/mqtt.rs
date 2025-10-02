use crate::prelude::*;
use esp_idf_svc::mqtt::client::{
    EspMqttClient, EspMqttConnection, EventPayload, MqttClientConfiguration, QoS,
};
use log::info;
use std::time::Duration;

const MQTT_URL: &str = env!("ESP_MQTT_URL");
const MQTT_CID: &str = env!("ESP_MQTT_CID");
const MQTT_TOPIC: &str = "green";
const MQTT_PAYLOAD: &str = r#"{status: "green"}"#;

pub struct Mqtt<'d> {
    conf: MqttClientConfiguration<'d>,
}

impl<'d> Mqtt<'d> {
    pub fn new() -> Result<Self> {
        Ok(Self {
            conf: MqttClientConfiguration {
                client_id: Some(MQTT_CID),
                ..Default::default()
            },
        })
    }

    pub fn start(&mut self) -> Result<()> {
        let (client, connection) = EspMqttClient::new(MQTT_URL, &self.conf)?;
        info!("MQTT listener starting");
        if let Err(e) = std::thread::Builder::new()
            .stack_size(1024 * 6)
            .spawn(|| listen(connection))
        {
            info!("MQTT listener error: {:?}", e);
        };

        info!("MQTT publisher starting");
        if let Err(e) = std::thread::Builder::new()
            .stack_size(1024 * 6)
            .spawn(move || publish(client))
        {
            info!("MQTT publisher error: {:?}", e);
        }
        Ok(())
    }
}

fn listen(mut connection: EspMqttConnection) {
    info!("MQTT listener running");
    while let Ok(event) = connection.next() {
        match event.payload() {
            EventPayload::Received {
                topic,
                data,
                details,
                ..
            } => {
                info!(
                    "MQTT listener received on topic {:?} with QoS {:?}: {}",
                    topic,
                    details,
                    str::from_utf8(data).unwrap_or("Invalid UTF-8")
                );
            }
            _ => {
                info!("MQTT listener event: {:?}", event.payload());
            }
        }
    }
    info!("MQTT listener stopped");
}

fn publish(mut client: EspMqttClient) {
    info!("MQTT publisher running");
    while let Err(e) = client.subscribe(MQTT_TOPIC, QoS::AtMostOnce) {
        info!("MQTT publisher subscribe error: {:?}", e);
        std::thread::sleep(Duration::from_millis(500));
    }
    info!("MQTT publisher subscribed");
    std::thread::sleep(Duration::from_millis(500));
    loop {
        if let Err(e) = client.enqueue(MQTT_TOPIC, QoS::AtMostOnce, false, MQTT_PAYLOAD.as_ref()) {
            info!("MQTT publisher enqueue error: {:?}", e);
            break;
        }
        std::thread::sleep(Duration::from_secs(2));
    }
}
