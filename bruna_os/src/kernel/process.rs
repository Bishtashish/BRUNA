// bruna_os/src/kernel/process.rs
use super::KernelResult;
use super::KernelError;
// Ensure KernelError is explicitly in scope
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use crate::kernel::scheduler::{RoundRobinScheduler, Scheduler}; // Moved HashMap import to the top

// Import thread-related items
use crate::kernel::thread::{Thread, ThreadId, ThreadState, generate_tid as generate_thread_id};
use crate::kernel::thread::ThreadManagement; // Added import for the trait itself

// Static counter for generating unique ProcessIds
static NEXT_PROCESS_ID: AtomicU64 = AtomicU64::new(1); // Start from 1, 0 could be special

pub type ProcessId = u64;

// Function to generate a new unique ProcessId
pub fn generate_pid() -> ProcessId {
    NEXT_PROCESS_ID.fetch_add(1, Ordering::Relaxed)
}

// Basic Process State
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    New,
    Ready,
    Running,
    Waiting,
    Terminated,
}

// Basic Process Control Block (PCB) structure
#[derive(Debug, Clone)] // Added Clone for easier management in SimpleProcessManager
pub struct Process {
    pub id: ProcessId,
    pub state: ProcessState,
    pub threads: HashMap<ThreadId, Thread>, // Added field for threads
    // pub parent_id: Option<ProcessId>,
    // pub priority: u8,
    // pub memory_space: (), // Placeholder for MemorySpace from memory.rs
}

impl Process {
    // Constructor for a new Process
    pub fn new(id: ProcessId) -> Self {
        Process {
            id,
            state: ProcessState::New, // Default state for a new process
            threads: HashMap::new(), // Initialize the new threads map
            // parent_id: None,
            // priority: 0,
            // memory_space: (),
        }
    }

    // New methods to be added:

    pub fn create_new_thread(&mut self) -> KernelResult<ThreadId> {
        let new_tid = generate_thread_id(); // Uses the imported aliased function
        // In a real scenario, one might check if new_tid somehow already exists in self.threads,
        // though a global atomic counter makes this extremely unlikely.
        if self.threads.contains_key(&new_tid) {
            return Err(KernelError::IPCError("Thread ID collision within process".to_string()));
        }
        let new_thread = Thread::new(new_tid, self.id.clone()); // self.id is the ProcessId
        self.threads.insert(new_tid, new_thread);
        Ok(new_tid)
    }

    pub fn terminate_existing_thread(&mut self, tid: ThreadId) -> KernelResult<()> {
        if self.threads.remove(&tid).is_some() {
            Ok(())
        } else {
            Err(KernelError::NotFound) // Thread not found in this process
        }
    }

    pub fn get_thread_state(&self, tid: ThreadId) -> KernelResult<ThreadState> {
        match self.threads.get(&tid) {
            Some(thread) => Ok(thread.state),
            None => Err(KernelError::NotFound), // Thread not found in this process
        }
    }

    pub fn set_thread_state(&mut self, tid: ThreadId, new_state: ThreadState) -> KernelResult<()> {
        match self.threads.get_mut(&tid) {
            Some(thread) => {
                thread.state = new_state;
                Ok(())
            }
            None => Err(KernelError::NotFound), // Thread not found in this process
        }
    }
}

pub trait ProcessManagement {
    // Updated to reflect that ProcessId is generated internally now
    fn create_process(&mut self /* args for the process, like a path to an executable or a function pointer */) -> KernelResult<ProcessId>;
    fn terminate_process(&mut self, pid: ProcessId) -> KernelResult<()>;
    fn get_process_state(&self, pid: ProcessId) -> KernelResult<ProcessState>;
    // fn list_processes() -> Vec<ProcessInfo>; // Could be Process struct itself or a summary
}

// Manages all active processes in the system
#[derive(Debug, Default)] // Default can be used for a simple new()
pub struct SimpleProcessManager {
    processes: HashMap<ProcessId, Process>,
    pub scheduler: RoundRobinScheduler, // Made scheduler field public for testing

    // We don't need to store next_pid here if we use the global static atomic NEXT_PROCESS_ID
    // and call generate_pid() when a process is created.
}

impl SimpleProcessManager {
    // Creates a new, empty SimpleProcessManager
    pub fn new() -> Self {
        SimpleProcessManager {
            processes: HashMap::new(),
            scheduler: RoundRobinScheduler::new(),
        }
    }
}

// Implementation of the ProcessManagement trait for SimpleProcessManager
impl ProcessManagement for SimpleProcessManager {
    fn create_process(&mut self) -> KernelResult<ProcessId> {
        let new_pid = generate_pid();
        let new_process = Process::new(new_pid);

        // It's good practice to ensure the PID isn't somehow already in use,
        // though with an atomic counter, this is highly unlikely for new PIDs.
        // For robustness, one might loop generate_pid until a truly unique one is found if the map already contained it,
        // but for this basic implementation, we assume generate_pid() is sufficient.
        if self.processes.contains_key(&new_pid) {
            // This case should ideally not happen with a monotonic global atomic counter
            // unless PIDs can be reused after termination and the counter wraps or isn't global.
            // For now, let's treat it as an unexpected error.
            return Err(super::KernelError::IPCError("PID collision, this should not happen".to_string()));
        }

        self.processes.insert(new_pid, new_process);
        Ok(new_pid)
    }

    fn terminate_process(&mut self, pid: ProcessId) -> KernelResult<()> {
        if self.processes.remove(&pid).is_some() {
            Ok(())
        } else {
            Err(super::KernelError::NotFound)
        }
    }

    fn get_process_state(&self, pid: ProcessId) -> KernelResult<ProcessState> {
        match self.processes.get(&pid) {
            Some(process) => Ok(process.state),
            None => Err(super::KernelError::NotFound),
        }
    }
}

// NEW IMPLEMENTATION BLOCK STARTS HERE
impl ThreadManagement for SimpleProcessManager {
    fn create_thread(&mut self, pid: ProcessId /*, _start_routine, _args */) -> KernelResult<ThreadId> {
        match self.processes.get_mut(&pid) {
            Some(process) => {
                // Now call the method on the Process struct
                let thread_result = process.create_new_thread();
                if let Ok(tid) = thread_result {
                    if process.get_thread_state(tid) == Ok(ThreadState::Ready) {
                        self.scheduler.add_thread(tid)?;
                    }
                }
                thread_result
            }
            None => Err(KernelError::NotFound), // Process not found
        }
    }

    fn terminate_thread(&mut self, pid: ProcessId, tid: ThreadId) -> KernelResult<()> {
        match self.processes.get_mut(&pid) {
            Some(process) => {
                let terminate_result = process.terminate_existing_thread(tid);
                if terminate_result.is_ok() {
                    self.scheduler.remove_thread(tid)?;
                }
                terminate_result
            }
            None => Err(KernelError::NotFound), // Process not found
        }
    }

    fn sleep_thread(&mut self, pid: ProcessId, tid: ThreadId, _duration_ms: u64) -> KernelResult<()> {
        // _duration_ms is ignored for now as we don't have actual sleep timers/scheduler interaction.
        // We just set the state to Blocked.
        match self.processes.get_mut(&pid) {
            Some(process) => {
                let sleep_result = process.set_thread_state(tid, ThreadState::Blocked);
                if sleep_result.is_ok() {
                    self.scheduler.remove_thread(tid)?;
                }
                sleep_result
            }
            None => Err(KernelError::NotFound), // Process not found
        }
    }

    fn get_thread_state(&self, pid: ProcessId, tid: ThreadId) -> KernelResult<ThreadState> {
        match self.processes.get(&pid) {
            Some(process) => {
                process.get_thread_state(tid)
            }
            None => Err(KernelError::NotFound), // Process not found
        }
    }
}
// NEW IMPLEMENTATION BLOCK ENDS HERE

#[cfg(test)]
mod tests {
    // use crate::kernel::scheduler::Scheduler;
    use super::*; // Imports SimpleProcessManager, ProcessManagement trait, ProcessState, KernelError, etc.

    #[test]
    fn test_create_process() {
        let mut manager = SimpleProcessManager::new();
        let result = manager.create_process();
        assert!(result.is_ok(), "Failed to create process");
        let pid = result.unwrap();
        assert_eq!(manager.get_process_state(pid).unwrap(), ProcessState::New);
    }

    #[test]
    fn test_create_multiple_processes_unique_pids() {
        let mut manager = SimpleProcessManager::new();
        let pid1 = manager.create_process().unwrap();
        let pid2 = manager.create_process().unwrap();
        assert_ne!(pid1, pid2);
    }

    #[test]
    fn test_terminate_existing_process() {
        let mut manager = SimpleProcessManager::new();
        let pid = manager.create_process().unwrap();
        assert!(manager.terminate_process(pid).is_ok());
        assert!(matches!(manager.get_process_state(pid), Err(KernelError::NotFound)));
    }

    #[test]
    fn test_terminate_non_existent_process() {
        let mut manager = SimpleProcessManager::new();
        assert!(matches!(manager.terminate_process(999), Err(KernelError::NotFound)));
    }

    // ThreadManagement tests from previous subtask
    #[test]
    fn test_create_thread_in_process() {
        let mut manager = SimpleProcessManager::new();
        let pid = manager.create_process().expect("Failed to create process");
        let tid = manager.create_thread(pid).expect("Failed to create thread");
        assert_eq!(manager.get_thread_state(pid, tid).unwrap(), ThreadState::Ready);
    }

    #[test]
    fn test_terminate_thread_in_process() {
        let mut manager = SimpleProcessManager::new();
        let pid = manager.create_process().unwrap();
        let tid = manager.create_thread(pid).unwrap();
        assert!(manager.terminate_thread(pid, tid).is_ok());
        assert!(matches!(manager.get_thread_state(pid, tid), Err(KernelError::NotFound)));
    }

    #[test]
    fn test_sleep_thread_in_process() {
        let mut manager = SimpleProcessManager::new();
        let pid = manager.create_process().unwrap();
        let tid = manager.create_thread(pid).unwrap();
        manager.sleep_thread(pid, tid, 100).unwrap();
        assert_eq!(manager.get_thread_state(pid, tid).unwrap(), ThreadState::Blocked);
    }

    // New tests focusing on scheduler integration:

    #[test]
    fn test_integration_create_thread_adds_to_scheduler() {
        let mut manager = SimpleProcessManager::new(); // SPM now has a scheduler
        let pid = manager.create_process().expect("Failed to create process");
        let tid = manager.create_thread(pid).expect("Failed to create thread");

        // More robust check by cycling through schedule_next:
        let mut temp_found = false;
        let queue_len = manager.scheduler.ready_queue.len(); // Max items to check
        for _ in 0..queue_len {
            if let Some(scheduled_tid) = manager.scheduler.schedule_next() {
                if scheduled_tid == tid {
                    temp_found = true;
                    break;
                }
            } else {
                break; // Queue empty
            }
        }
        assert!(temp_found, "Newly created ready thread (tid: {}) should be schedulable", tid);
    }

    #[test]
    fn test_integration_terminate_thread_removes_from_scheduler() {
        let mut manager = SimpleProcessManager::new();
        let pid = manager.create_process().unwrap();
        let tid = manager.create_thread(pid).unwrap();

        assert!(manager.scheduler.ready_queue.contains(&tid), "Thread should be in scheduler before termination");
        manager.terminate_thread(pid, tid).expect("Failed to terminate thread");
        assert!(!manager.scheduler.ready_queue.contains(&tid), "Terminated thread should be removed from the scheduler's ready queue");
    }

    #[test]
    fn test_integration_sleep_thread_removes_from_scheduler() {
        let mut manager = SimpleProcessManager::new();
        let pid = manager.create_process().unwrap();
        let tid = manager.create_thread(pid).unwrap();

        assert!(manager.scheduler.ready_queue.contains(&tid), "Thread should be in scheduler before sleep");
        manager.sleep_thread(pid, tid, 100).expect("Failed to sleep thread");
        assert!(!manager.scheduler.ready_queue.contains(&tid), "Sleeping thread should be removed from the scheduler's ready queue");
        assert_eq!(manager.get_thread_state(pid, tid).unwrap(), ThreadState::Blocked, "Thread should be in Blocked state");
    }

    #[test]
    fn test_integration_scheduler_handles_multiple_threads() {
        let mut manager = SimpleProcessManager::new();
        let pid = manager.create_process().unwrap();
        let tid1 = manager.create_thread(pid).unwrap();
        let tid2 = manager.create_thread(pid).unwrap();

        assert_eq!(manager.scheduler.ready_queue.len(), 2, "Scheduler should have 2 threads");
        assert!(manager.scheduler.ready_queue.contains(&tid1));
        assert!(manager.scheduler.ready_queue.contains(&tid2));

        let first_scheduled = manager.scheduler.schedule_next().unwrap();
        assert_eq!(manager.scheduler.ready_queue.len(), 2, "Scheduler should still have 2 threads after one schedule_next (round-robin re-adds)");

        manager.terminate_thread(pid, first_scheduled).unwrap();
        assert_eq!(manager.scheduler.ready_queue.len(), 1, "Scheduler should have 1 thread after termination");
        assert!(!manager.scheduler.ready_queue.contains(&first_scheduled));

        let remaining_tid = if first_scheduled == tid1 { tid2 } else { tid1 };
        assert!(manager.scheduler.ready_queue.contains(&remaining_tid));
    }
}
