// bruna_os/src/hal/timers.rs
use super::common::HalResult;
use std::time::Duration;

pub trait Timer {
    type TimerId;

    fn new(id: Self::TimerId) -> HalResult<Self> where Self: Sized;
    fn start(&mut self, duration: Duration, periodic: bool, callback: Box<dyn FnMut() + Send>) -> HalResult<()>;
    fn stop(&mut self) -> HalResult<()>;
    // fn get_remaining_time(&self) -> HalResult<Duration>;
}
