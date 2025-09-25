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
}

pub struct Text<'a>(&'a str);

impl<'a> From<&'a str> for Text<'a> {
    fn from(value: &'a str) -> Self {
        Self(value)
    }
}

impl<'a, const N: usize> TryFrom<Text<'a>> for heapless::String<N> {
    type Error = Error;

    fn try_from(value: Text) -> std::result::Result<Self, Self::Error> {
        heapless::String::try_from(value.0).map_err(|_| {
            log::error!("Unable to convert string: {:#?}", value.0);
            Error::StringError()
        })
    }
}