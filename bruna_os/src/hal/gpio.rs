// bruna_os/src/hal/gpio.rs
use super::common::{HalResult, HalError};

#[derive(Debug, Clone, Copy)]
pub enum PinMode {
    Input,
    Output,
    InputPullUp,
    InputPullDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinState {
    Low,
    High,
}

pub trait GpioPin {
    type PinIdentifier; // e.g., u8 for pin number, or a string

    fn new(identifier: Self::PinIdentifier) -> HalResult<Self> where Self: Sized;
    fn set_mode(&mut self, mode: PinMode) -> HalResult<()>;
    fn read(&self) -> HalResult<PinState>;
    fn write(&mut self, state: PinState) -> HalResult<()>;
    // fn toggle(&mut self) -> HalResult<()>;
    // fn set_interrupt_handler(&mut self, handler: Box<dyn Fn(PinState) + Send + Sync>) -> HalResult<()>;
}
