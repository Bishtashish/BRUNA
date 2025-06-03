// bruna_os/src/hal/mod.rs
pub mod common;
pub mod serial;
pub mod gpio;
pub mod timers;
pub mod network;
pub mod radio; // For generic radio communication like nRF24

// Re-export common types or traits if desired
pub use common::{HardwareId, HalError, HalResult};
pub use serial::SerialDevice;
pub use gpio::{GpioPin, PinMode, PinState};
pub use timers::Timer;
pub use network::NetworkInterface;
pub use radio::RadioDevice;

pub mod platforms;

// A generic trait that all platform-specific HALs might implement
pub trait PlatformHal {
    type Serial: SerialDevice;
    type Gpio: GpioPin; // This might be a collection of pins
    type Timer: Timer;
    type Network: NetworkInterface;
    type Radio: RadioDevice;

    fn new() -> Self; // Or some platform specific init
    fn platform_name(&self) -> &'static str;
}
