// bruna_os/src/hal/serial.rs
use super::common::{HalResult, HalError};

pub trait SerialDevice {
    fn open(port: &str, baud_rate: u32) -> HalResult<Self> where Self: Sized;
    fn read(&mut self, buffer: &mut [u8]) -> HalResult<usize>;
    fn write(&mut self, data: &[u8]) -> HalResult<usize>;
    fn close(self) -> HalResult<()>;
    // fn set_timeout(&mut self, timeout_ms: u32) -> HalResult<()>;
}
