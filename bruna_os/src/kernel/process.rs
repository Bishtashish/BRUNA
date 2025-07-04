// bruna_os/src/kernel/process.rs
use super::KernelResult;
use super::KernelError;
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;

use crate::kernel::thread::{Thread, ThreadId, ThreadState, generate_tid as generate_thread_id};
use crate::kernel::thread::ThreadManagement;
use crate::kernel::scheduler::{RoundRobinScheduler, Scheduler};
use crate::kernel::ipc::{SystemMessageBus, Message, MessagePassing, MessageId};

static NEXT_PROCESS_ID: AtomicU64 = AtomicU64::new(1);
pub type ProcessId = u64;

pub fn generate_pid() -> ProcessId {
    NEXT_PROCESS_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    New, Ready, Running, Waiting, Terminated,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: ProcessId,
    pub state: ProcessState,
    pub threads: HashMap<ThreadId, Thread>,
}

impl Process {
    pub fn new(id: ProcessId) -> Self {
        Process {
            id,
            state: ProcessState::New,
            threads: HashMap::new(),
        }
    }

    pub fn create_new_thread(&mut self) -> KernelResult<ThreadId> {
        let new_tid = generate_thread_id();
        if self.threads.contains_key(&new_tid) {
            return Err(KernelError::Other("Thread ID collision within process".to_string()));
        }
        let new_thread = Thread::new(new_tid, self.id);
        self.threads.insert(new_tid, new_thread);
        Ok(new_tid)
    }

    pub fn terminate_existing_thread(&mut self, tid: ThreadId) -> KernelResult<()> {
        if self.threads.remove(&tid).is_some() { Ok(()) } else { Err(KernelError::NotFound) }
    }

    pub fn get_thread_state(&self, tid: ThreadId) -> KernelResult<ThreadState> {
        match self.threads.get(&tid) {
            Some(thread) => Ok(thread.state),
            None => Err(KernelError::NotFound),
        }
    }

    pub fn set_thread_state(&mut self, tid: ThreadId, new_state: ThreadState) -> KernelResult<()> {
        match self.threads.get_mut(&tid) {
            Some(thread) => {
                thread.state = new_state;
                Ok(())
            }
            None => Err(KernelError::NotFound),
        }
    }
}

pub trait ProcessManagement {
    fn create_process(&mut self) -> KernelResult<ProcessId>;
    fn terminate_process(&mut self, pid: ProcessId) -> KernelResult<()>;
    fn get_process_state(&self, pid: ProcessId) -> KernelResult<ProcessState>;
}

#[derive(Debug)]
pub struct SimpleProcessManager {
    processes: HashMap<ProcessId, Process>,
    pub scheduler: RoundRobinScheduler, // Existing field
    ipc_bus: SystemMessageBus,          // Added IPC bus field
}

impl SimpleProcessManager {
    pub fn new() -> Self {
        SimpleProcessManager {
            processes: HashMap::new(),
            scheduler: RoundRobinScheduler::new(),
            ipc_bus: SystemMessageBus::new(), // Initialize ipc_bus
        }
    }
}

impl ProcessManagement for SimpleProcessManager {
    fn create_process(&mut self) -> KernelResult<ProcessId> {
        let new_pid = generate_pid();
        let new_process = Process::new(new_pid);
        if self.processes.contains_key(&new_pid) {
            return Err(KernelError::Other("PID collision".to_string()));
        }
        self.processes.insert(new_pid, new_process);
        Ok(new_pid)
    }

    fn terminate_process(&mut self, pid: ProcessId) -> KernelResult<()> {
        if self.processes.remove(&pid).is_some() { Ok(()) } else { Err(KernelError::NotFound) }
    }

    fn get_process_state(&self, pid: ProcessId) -> KernelResult<ProcessState> {
        match self.processes.get(&pid) {
            Some(process) => Ok(process.state),
            None => Err(KernelError::NotFound),
        }
    }
}

impl ThreadManagement for SimpleProcessManager {
    fn create_thread(&mut self, pid: ProcessId) -> KernelResult<ThreadId> {
        match self.processes.get_mut(&pid) {
            Some(process) => {
                let thread_result = process.create_new_thread();
                if let Ok(tid) = thread_result {
                    if process.get_thread_state(tid) == Ok(ThreadState::Ready) {
                        self.scheduler.add_thread(tid)?;
                    }
                }
                thread_result
            }
            None => Err(KernelError::NotFound),
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
            None => Err(KernelError::NotFound),
        }
    }

    fn sleep_thread(&mut self, pid: ProcessId, tid: ThreadId, _duration_ms: u64) -> KernelResult<()> {
        match self.processes.get_mut(&pid) {
            Some(process) => {
                let sleep_result = process.set_thread_state(tid, ThreadState::Blocked);
                if sleep_result.is_ok() {
                    self.scheduler.remove_thread(tid)?;
                }
                sleep_result
            }
            None => Err(KernelError::NotFound),
        }
    }

    fn get_thread_state(&self, pid: ProcessId, tid: ThreadId) -> KernelResult<ThreadState> {
        match self.processes.get(&pid) {
            Some(process) => process.get_thread_state(tid),
            None => Err(KernelError::NotFound),
        }
    }
}

// New: Implement MessagePassing for SimpleProcessManager by delegating to ipc_bus
impl MessagePassing for SimpleProcessManager {
    fn send_message(&mut self, message: Message) -> KernelResult<()> {
        self.ipc_bus.send_message(message)
    }

    fn receive_message(&mut self, receiver_pid: ProcessId) -> KernelResult<Message> {
        self.ipc_bus.receive_message(receiver_pid)
    }

    fn try_receive_message(&mut self, receiver_pid: ProcessId) -> KernelResult<Option<Message>> {
        self.ipc_bus.try_receive_message(receiver_pid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    // New tests focusing on IPC integration via SimpleProcessManager:

    #[test]
    fn test_spm_ipc_send_receive_between_processes() {
        let mut manager = SimpleProcessManager::new();
        let pid1 = manager.create_process().expect("Failed to create process 1");
        let pid2 = manager.create_process().expect("Failed to create process 2");

        let payload_p1_to_p2 = vec![1, 2, 3, 4, 5];
        let message_p1 = Message::new(pid1, pid2, payload_p1_to_p2.clone());
        let msg1_id = message_p1.id;

        // SPM now implements MessagePassing, so we call it directly on manager
        assert!(manager.send_message(message_p1).is_ok(), "Process 1 failed to send message");

        // Process 2 receives the message
        let received_message_result = manager.receive_message(pid2);
        assert!(received_message_result.is_ok(), "Process 2 failed to receive message: {:?}", received_message_result.err());
        let received_message = received_message_result.unwrap();

        assert_eq!(received_message.id, msg1_id);
        assert_eq!(received_message.sender_pid, pid1);
        assert_eq!(received_message.receiver_pid, pid2);
        assert_eq!(received_message.payload, payload_p1_to_p2);
    }

    #[test]
    fn test_spm_ipc_try_receive_no_message() {
        let mut manager = SimpleProcessManager::new();
        let pid1 = manager.create_process().unwrap(); // Process that will try to receive

        let result = manager.try_receive_message(pid1);
        assert!(result.is_ok(), "try_receive_message failed");
        assert!(result.unwrap().is_none(), "Expected no message for pid1");
    }

    #[test]
    fn test_spm_ipc_receive_no_message_error() {
        let mut manager = SimpleProcessManager::new();
        let pid1 = manager.create_process().unwrap();

        let result = manager.receive_message(pid1);
        assert!(result.is_err(), "receive_message should fail for empty queue");
        assert_eq!(result.err().unwrap(), KernelError::NotFound); // Or specific NoMessage error
    }

    #[test]
    fn test_spm_ipc_multiple_messages_fifo() {
        let mut manager = SimpleProcessManager::new();
        let p_sender = manager.create_process().unwrap();
        let p_receiver = manager.create_process().unwrap();

        let msg_payload1 = vec![10];
        let msg_payload2 = vec![20];

        let msg1 = Message::new(p_sender, p_receiver, msg_payload1.clone());
        let msg2 = Message::new(p_sender, p_receiver, msg_payload2.clone());
        let msg1_id = msg1.id;
        let msg2_id = msg2.id;

        manager.send_message(msg1).unwrap();
        manager.send_message(msg2).unwrap();

        let recv_msg1 = manager.receive_message(p_receiver).unwrap();
        assert_eq!(recv_msg1.id, msg1_id);
        assert_eq!(recv_msg1.payload, msg_payload1);

        let recv_msg2 = manager.receive_message(p_receiver).unwrap();
        assert_eq!(recv_msg2.id, msg2_id);
        assert_eq!(recv_msg2.payload, msg_payload2);
    }
}
