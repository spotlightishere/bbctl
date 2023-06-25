use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub enum DeviceError {
    NoDeviceFound,
    ProtocolError,
    USBError(rusb::Error),
}

impl Display for DeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            Self::NoDeviceFound => write!(f, "no BlackBerry was detected"),
            Self::ProtocolError => write!(f, "a protocol error was detected"),
            Self::USBError(e) => write!(f, "an USB error was encountered: {}", e),
        }
    }
}

impl Error for DeviceError {}

impl From<rusb::Error> for DeviceError {
    fn from(rusb_err: rusb::Error) -> Self {
        DeviceError::USBError(rusb_err)
    }
}
