// bruna_os/src/hal/common.rs

// Unique identifier for a piece of hardware
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HardwareId(pub String); // Could be MAC address, serial number, etc.

#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Trait for non-volatile storage access
pub trait Storage {
    fn read(&self, offset: usize, buf: &mut [u8]) -> HalResult<usize>;
    fn write(&mut self, offset: usize, buf: &[u8]) -> HalResult<usize>;
    fn size(&self) -> usize;
}
