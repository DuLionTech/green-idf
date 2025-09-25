use esp_idf_svc::sys::EspError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("System Error: {0:?}")]
    EspError(#[from] EspError),

    #[error("String Error {0}")]
    StringError(&'static str),
}

pub struct Text(&'static str);

impl Text {
    pub const fn new(text: &'static str) -> Self {
        Self(text)
    }
}

impl<const N: usize> TryFrom<Text> for heapless::String<N> {
    type Error = Error;

    fn try_from(value: Text) -> std::result::Result<Self, Self::Error> {
        let str = value.0;
        heapless::String::try_from(str).map_err(|_| Error::StringError(str))
    }
}