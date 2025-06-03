// bruna_os/src/kernel/thread.rs
use super::process::ProcessId;
use super::KernelResult; // Assuming KernelError is handled via this or imported separately
use std::sync::atomic::{AtomicU64, Ordering};

// Static counter for generating unique ThreadIds globally
static NEXT_THREAD_ID: AtomicU64 = AtomicU64::new(1);

pub type ThreadId = u64;

// Function to generate a new unique ThreadId
pub fn generate_tid() -> ThreadId {
    NEXT_THREAD_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    Ready,    // Ready to run
    Running,  // Currently executing
    Blocked,  // Waiting for an event (e.g., I/O, semaphore, sleep)
    Terminated, // Execution finished
}

#[derive(Debug, Clone, PartialEq, Eq)] // Added PartialEq, Eq

pub struct Thread {
    pub id: ThreadId,
    pub process_id: ProcessId,
    pub state: ThreadState,
    // pub stack_pointer: usize, // Placeholder for future use
    // pub instruction_pointer: usize, // Placeholder for future use
    // pub priority: u8, // Placeholder for future use
    // context: Option<ThreadContext>, // For context switching
}

impl Thread {
    // Constructor for a new Thread
    pub fn new(id: ThreadId, process_id: ProcessId) -> Self {
        Thread {
            id,
            process_id,
            state: ThreadState::Ready, // Default state for a new thread
            // stack_pointer: 0,
            // instruction_pointer: 0,
            // priority: 0,
            // context: None,
        }
    }
}

// Trait for thread management operations.
// This will be implemented by SimpleProcessManager (or a dedicated ThreadManager).
// Signatures will be reviewed and potentially updated in a later step.
pub trait ThreadManagement {
    fn create_thread(&mut self, pid: ProcessId /*, start_routine, args */) -> KernelResult<ThreadId>;
    fn terminate_thread(&mut self, pid: ProcessId, tid: ThreadId) -> KernelResult<()>;
    fn sleep_thread(&mut self, pid: ProcessId, tid: ThreadId, duration_ms: u64) -> KernelResult<()>;
    fn get_thread_state(&self, pid: ProcessId, tid: ThreadId) -> KernelResult<ThreadState>;

    // fn yield_thread(); // Might be handled by scheduler
    // fn join_thread(tid: ThreadId) -> KernelResult<()>; // For waiting for a thread to finish
}
