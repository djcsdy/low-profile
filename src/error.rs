use crate::http::InvalidMethod;

#[derive(Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum InvalidUrl {
    TooLong,
    InvalidUrlCodePoint,
}

#[derive(Debug)]
pub enum ProtocolError {
    InvalidUrl(InvalidUrl),
    InvalidMethod(InvalidMethod),
    Parser(httparse::Error),
}

#[cfg(feature = "defmt")]
impl defmt::Format for ProtocolError {
    fn format(&self, f: defmt::Formatter) {
        match self {
            ProtocolError::InvalidUrl(invalid_url) => defmt::write!(f, "{:?}", invalid_url),
            ProtocolError::InvalidMethod(invalid_method) => defmt::write!(f, "{:?}", invalid_method),
            ProtocolError::Parser(error) => match error {
                httparse::Error::HeaderName => defmt::write!(f, "HeaderName"),
                httparse::Error::HeaderValue => defmt::write!(f, "HeaderValue"),
                httparse::Error::NewLine => defmt::write!(f, "NewLine"),
                httparse::Error::Status => defmt::write!(f, "Status"),
                httparse::Error::Token => defmt::write!(f, "Token"),
                httparse::Error::TooManyHeaders => defmt::write!(f, "TooManyHeaders"),
                httparse::Error::Version => defmt::write!(f, "Version"),
            }
        }
    }
}