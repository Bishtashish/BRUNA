// bruna_os/src/kernel/ipc.rs
use super::KernelResult;
use super::process::ProcessId;

pub type MessageId = u64;

// Basic message structure
pub struct Message {
    pub id: MessageId,
    pub sender_pid: ProcessId,
    pub receiver_pid: ProcessId,
    pub payload: Vec<u8>, // Simple byte payload for now
    // pub timestamp: u64,
}

// Example IPC mechanism: Message Passing
pub trait MessagePassing {
    fn send_message(message: Message) -> KernelResult<()>;
    fn receive_message(receiver_pid: ProcessId) -> KernelResult<Message>; // Blocking receive
    // fn try_receive_message(receiver_pid: ProcessId) -> KernelResult<Option<Message>>; // Non-blocking
}

// Other IPC mechanisms like Semaphores, Mutexes, Shared Memory could be defined here later
