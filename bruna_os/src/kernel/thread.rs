// bruna_os/src/kernel/thread.rs
use super::process::ProcessId;
use super::KernelResult;

pub type ThreadId = u64; // Or a more complex type

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready,
    Running,
    Blocked,
    Terminated,
}

pub struct Thread {
    pub id: ThreadId,
    pub process_id: ProcessId,
    pub state: ThreadState,
    // pub stack_pointer: usize,
    // pub instruction_pointer: usize,
    // pub priority: u8,
}

pub trait ThreadManagement {
    fn create_thread(pid: ProcessId /*, start_routine, args */) -> KernelResult<ThreadId>;
    fn terminate_thread(tid: ThreadId) -> KernelResult<()>;
    fn sleep_thread(tid: ThreadId, duration_ms: u64) -> KernelResult<()>;
    // fn yield_thread();
}
