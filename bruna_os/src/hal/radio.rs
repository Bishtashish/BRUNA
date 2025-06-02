// bruna_os/src/hal/radio.rs
use super::common::{HalResult, HalError};

// Generic trait for radio transceivers like nRF24, LoRa, etc.
pub trait RadioDevice {
    fn new(/* config parameters */) -> HalResult<Self> where Self: Sized;
    fn set_channel(&mut self, channel: u8) -> HalResult<()>;
    fn set_datarate(&mut self, datarate: &str) -> HalResult<()>; // e.g., "250kbps", "1Mbps"
    fn set_tx_power(&mut self, power_level: i8) -> HalResult<()>;
    fn transmit(&mut self, payload: &[u8]) -> HalResult<()>;
    fn receive(&mut self, buffer: &mut [u8]) -> HalResult<usize>; // Returns number of bytes received
    // fn listen_for_packet_async(&mut self, callback: Box<dyn Fn(&[u8]) + Send>) -> HalResult<()>;
}
