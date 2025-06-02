// bruna_os/src/kernel/process.rs
use super::KernelResult;
use super::KernelError; // Ensure KernelError is explicitly in scope
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap; // Moved HashMap import to the top

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
    // pub parent_id: Option<ProcessId>,
    // pub priority: u8,
    // pub memory_space: (), // Placeholder for MemorySpace from memory.rs
    // pub threads: Vec<()>,   // Placeholder for ThreadId from thread.rs
}

impl Process {
    // Constructor for a new Process
    pub fn new(id: ProcessId) -> Self {
        Process {
            id,
            state: ProcessState::New, // Default state for a new process
            // parent_id: None,
            // priority: 0,
            // memory_space: (),
            // threads: Vec::new(),
        }
    }
}

pub trait ProcessManagement {
    // Updated to reflect that ProcessId is generated internally now
    fn create_process(/* args for the process, like a path to an executable or a function pointer */) -> KernelResult<ProcessId>;
    fn terminate_process(pid: ProcessId) -> KernelResult<()>;
    fn get_process_state(pid: ProcessId) -> KernelResult<ProcessState>;
    // fn list_processes() -> Vec<ProcessInfo>; // Could be Process struct itself or a summary
}

// Manages all active processes in the system
#[derive(Debug, Default)] // Default can be used for a simple new()
pub struct SimpleProcessManager {
    processes: HashMap<ProcessId, Process>,
    // We don't need to store next_pid here if we use the global static atomic NEXT_PROCESS_ID
    // and call generate_pid() when a process is created.
}

impl SimpleProcessManager {
    // Creates a new, empty SimpleProcessManager
    pub fn new() -> Self {
        SimpleProcessManager {
            processes: HashMap::new(),
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
            return Err(super::KernelError::Other("PID collision, this should not happen".to_string()));
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

#[cfg(test)]
mod tests {
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
        let pid1_result = manager.create_process();
        assert!(pid1_result.is_ok());
        let pid1 = pid1_result.unwrap();

        let pid2_result = manager.create_process();
        assert!(pid2_result.is_ok());
        let pid2 = pid2_result.unwrap();

        assert_ne!(pid1, pid2, "Process IDs should be unique");
        assert_eq!(manager.get_process_state(pid1).unwrap(), ProcessState::New);
        assert_eq!(manager.get_process_state(pid2).unwrap(), ProcessState::New);
    }

    #[test]
    fn test_terminate_existing_process() {
        let mut manager = SimpleProcessManager::new();
        let pid = manager.create_process().unwrap();
        
        let terminate_result = manager.terminate_process(pid);
        assert!(terminate_result.is_ok(), "Failed to terminate existing process");
        
        // Verify process is gone
        match manager.get_process_state(pid) {
            Err(KernelError::NotFound) => { /* Expected */ },
            Ok(_) => panic!("Process state was found after termination"),
            Err(_) => panic!("Unexpected error type after termination"),
        }
        
        // Also check internal map size if possible, or try terminating again
        let terminate_again_result = manager.terminate_process(pid);
        match terminate_again_result {
            Err(KernelError::NotFound) => { /* Expected */ },
            Ok(_) => panic!("Terminating a non-existent process should fail"),
            Err(_) => panic!("Unexpected error type on second termination attempt"),
        }
    }

    #[test]
    fn test_terminate_non_existent_process() {
        let mut manager = SimpleProcessManager::new();
        let non_existent_pid: ProcessId = 999; // Assuming this PID won't exist
        
        let terminate_result = manager.terminate_process(non_existent_pid);
        match terminate_result {
            Err(KernelError::NotFound) => { /* Expected */ },
            Ok(_) => panic!("Should not be able to terminate a non-existent process"),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_get_process_state_existing() {
        let mut manager = SimpleProcessManager::new();
        let pid = manager.create_process().unwrap();
        
        let state_result = manager.get_process_state(pid);
        assert!(state_result.is_ok(), "Failed to get state for existing process");
        assert_eq!(state_result.unwrap(), ProcessState::New);
    }

    #[test]
    fn test_get_process_state_non_existent() {
        let mut manager = SimpleProcessManager::new();
        let non_existent_pid: ProcessId = 999;
        
        let state_result = manager.get_process_state(non_existent_pid);
        match state_result {
            Err(KernelError::NotFound) => { /* Expected */ },
            Ok(_) => panic!("Should not be able to get state for a non-existent process"),
            Err(e) => panic!("Unexpected error type: {:?}", e),
        }
    }
    
    #[test]
    fn test_pid_collision_is_handled() {
        // This test is a bit tricky because NEXT_PROCESS_ID is global and atomic.
        // To reliably test the collision logic in create_process, we'd need to mock
        // generate_pid or manipulate the internal state of SimpleProcessManager in a way
        // that's not currently exposed.
        // For now, we acknowledge the check exists in create_process.
        // A more advanced test might involve creating a process, terminating it,
        // somehow resetting NEXT_PROCESS_ID (not possible with static AtomicU64 easily from test)
        // or pre-populating the map.
        // Given the current implementation, direct testing of the collision `Err` path
        // is difficult without refactoring for testability.
        // We primarily rely on the atomic counter preventing this.
        // So, this test can be a placeholder or focus on the normal unique PID generation.
        let mut manager = SimpleProcessManager::new();
        // Create many processes to ensure the counter works
        for _ in 0..100 {
            assert!(manager.create_process().is_ok());
        }
        // This doesn't test the collision error path directly but exercises the PID generation.
    }
}
