use core::future::Future;

use crate::{error::ProtocolError, Read, Write};

#[derive(Debug)]
pub enum ServiceError<IO, BODY> {
    ProtocolError(ProtocolError),
    Io(IO),
    Body(BODY),
}

impl<IO: embedded_io_async::Error, BODY: embedded_io_async::Error> embedded_io_async::Error
    for ServiceError<IO, BODY>
{
    fn kind(&self) -> embedded_io_async::ErrorKind {
        match self {
            Self::Io(err) => err.kind(),
            Self::Body(err) => err.kind(),
            Self::ProtocolError(..) => embedded_io_async::ErrorKind::Other,
        }
    }
}

// defmt::Format is not implemented for embedded_io_async::Error, so provide our own implementation.
#[cfg(feature = "defmt")]
impl<IO: embedded_io_async::Error, BODY: embedded_io_async::Error> defmt::Format for ServiceError<IO, BODY> {
    fn format(&self, f: defmt::Formatter) {
        match self {
            ServiceError::ProtocolError(err) => defmt::write!(f, "{}", err),
            ServiceError::Io(err) => defmt::write!(f, "{}", err.kind()),
            ServiceError::Body(err) => defmt::write!(f, "{}", err.kind()),
        }
    }
}

pub trait Service {
    // TODO: this should come from crate::io or somewhere else
    type BodyError: embedded_io_async::Error;

    fn serve<R: Read<Error = E>, W: Write<Error = E>, E>(
        &self,
        reader: R,
        writer: W,
    ) -> impl Future<Output = Result<(), ServiceError<E, Self::BodyError>>>;
}
