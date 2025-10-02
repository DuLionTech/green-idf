use esp_idf_svc::io::EspIOError;
use esp_idf_svc::sys::EspError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("System Error: {0:?}")]
    EspError(#[from] EspError),

    #[error("IO Error: {0:?}")]
    EspIOError(#[from] EspIOError),

    #[error("String conversion failed")]
    StringError(),

    #[error("Error: {0:?}")]
    Error(&'static str),
}

pub fn to_string<const N: usize>(value: &str) -> Result<heapless::String<N>> {
    heapless::String::try_from(value).map_err(|_| {
        log::error!("Unable to convert string: {:#?}", value);
        Error::StringError()
    })
}