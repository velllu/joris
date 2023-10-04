use std::io;

#[derive(Debug)]
pub enum JorisError {
    CouldNotConnect,
    CouldNotParseResponse,
}

impl From<ureq::Error> for JorisError {
    fn from(_: ureq::Error) -> Self {
        Self::CouldNotConnect
    }
}

impl From<io::Error> for JorisError {
    fn from(_: io::Error) -> Self {
        Self::CouldNotParseResponse
    }
}
