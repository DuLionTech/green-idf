use crate::utils::Result;
use esp_idf_hal::io::Write;
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::http::Method;

const STACK_SIZE: usize = 10_240;
static INDEX_HTML: &str = include_str!("index.html");

pub struct Http<'d> {
    server: EspHttpServer<'d>,
}

impl<'d> Http<'d> {
    pub fn new() -> Result<Self> {
        let server_configuration = esp_idf_svc::http::server::Configuration {
            stack_size: STACK_SIZE,
            ..Default::default()
        };

        let server = EspHttpServer::new(&server_configuration)?;
        Ok(Self { server })
    }

    pub fn start(&mut self) -> Result<()> {
        (&mut self.server).fn_handler("/", Method::Get, |req| {
            req.into_ok_response()?.write_all(INDEX_HTML.as_bytes())
        })?;
        Ok(())
    }
}
