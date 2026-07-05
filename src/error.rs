#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum CoolTekSdkError {
    FailedToAllocateMemory = 0x01,
    TransferFailed = 0x03,
    InvalidResponse = 0x04,
    OperationFailed = 0x05,
    InvalidParameter = 0x06,
    DeviceNotConnected = 0x08,
    DeviceFailedToOpen = 0x09,
    DeviceAlreadyConnected = 0x0a,
    PayloadTooLarge = 0x0b,
}

impl std::error::Error for CoolTekSdkError {}

impl std::fmt::Display for CoolTekSdkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FailedToAllocateMemory => write!(f, "Failed to allocate memory."),
            Self::TransferFailed => write!(f, "Failed to transfer data."),
            Self::InvalidResponse => write!(f, "Received an invalid response."),
            Self::OperationFailed => write!(f, "The operation failed."),
            Self::InvalidParameter => write!(f, "Received a parameter with an invalid value."),
            Self::DeviceNotConnected => write!(f, "The HID is not connected."),
            Self::DeviceFailedToOpen => write!(f, "Failed to open the HID."),
            Self::DeviceAlreadyConnected => write!(f, "The HID is already connected."),
            Self::PayloadTooLarge => write!(f, "The payload is too large for the HID."),
        }
    }
}

pub type CoolTekSdkResult<T> = std::result::Result<T, CoolTekSdkError>;

// -------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RcvError {
    NullHandle,
    DeviceNotConnected,
    Timeout,
    HidReadError,
    BadReportId,
    UnknownTag,
    Routed,
    DrainError,
}

impl From<RcvError> for u8 {
    fn from(value: RcvError) -> Self {
        match value {
            RcvError::BadReportId => 4,
            RcvError::DeviceNotConnected => 1,
            RcvError::DrainError => 4,
            RcvError::HidReadError => 7,
            RcvError::NullHandle => 8,
            RcvError::Routed => 6,
            RcvError::Timeout => 2,
            RcvError::UnknownTag => 4,
        }
    }
}

impl std::error::Error for RcvError {}

impl std::fmt::Display for RcvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadReportId => write!(f, "The device returned an unexpected report ID."),
            Self::DeviceNotConnected => write!(f, "The device is not connected."),
            Self::DrainError => write!(f, "An error occurred trying to drain the ring buffer."),
            Self::HidReadError => {
                write!(f, "An error occurred trying to read data from the device.")
            }
            Self::NullHandle => write!(
                f,
                "The device handle was null, which should not be possible."
            ),
            Self::Routed => write!(f, "The data was routed to the ring buffer."),
            Self::Timeout => write!(f, "A timeout occurred waiting for the device to respond."),
            Self::UnknownTag => write!(f, "An unknown tag was found in the data."),
        }
    }
}
