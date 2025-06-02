// bruna_os/src/kernel/scheduler.rs
use super::thread::{ThreadId, Thread};
use super::process::{ProcessId, Process};
use super::KernelResult;

pub trait Scheduler {
    fn schedule_next(&mut self) -> Option<ThreadId>; // Returns the next thread to run
    fn add_thread(&mut self, thread: Thread) -> KernelResult<()>;
    fn remove_thread(&mut self, tid: ThreadId) -> KernelResult<()>;
    // fn set_priority(tid: ThreadId, priority: u8) -> KernelResult<()>;
}

// Example: A simple Round Robin scheduler (conceptual)
// pub struct RoundRobinScheduler {
//     ready_queue: std::collections::VecDeque<ThreadId>,
// }
