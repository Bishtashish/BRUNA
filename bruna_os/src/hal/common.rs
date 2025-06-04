// bruna_os/src/hal/common.rs

// Unique identifier for a piece of hardware
pub struct HardwareId(pub String); // Could be MAC address, serial number, etc.

#[derive(Debug)]
pub enum HalError {
    DeviceNotFound,
    InitializationFailed,
    ReadError(String),
    WriteError(String),
    UnsupportedOperation,
    InvalidParameter(String),
    Other(String),
}

pub type HalResult<T> = Result<T, HalError>;
