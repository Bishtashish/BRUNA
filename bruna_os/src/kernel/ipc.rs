// bruna_os/src/kernel/ipc.rs
use crate::kernel::process::ProcessId;
use crate::kernel::KernelResult;
// It's good practice to also import KernelError if specific errors from it are constructed here,
// or if it's not re-exported by `crate::kernel::KernelResult`. Assume `KernelResult` is enough for now.
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::{HashMap, VecDeque}; // Make sure these are imported
use crate::kernel::KernelError; // For returning specific errors

// Static counter for generating unique MessageIds globally
static NEXT_MESSAGE_ID: AtomicU64 = AtomicU64::new(1);

pub type MessageId = u64;

// Function to generate a new unique MessageId
pub fn generate_mid() -> MessageId {
    NEXT_MESSAGE_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone)] // Added Clone for easier handling
pub struct Message {
    pub id: MessageId,
    pub sender_pid: ProcessId,
    pub receiver_pid: ProcessId,
    pub payload: Vec<u8>, // Simple byte payload for now
    // pub timestamp: u64, // Optional: for future use
}

impl Message {
    // Constructor for a new Message
    pub fn new(sender_pid: ProcessId, receiver_pid: ProcessId, payload: Vec<u8>) -> Self {
        Message {
            id: generate_mid(), // Generate ID automatically
            sender_pid,
            receiver_pid,
            payload,
            // timestamp: 0, // If timestamp field is added
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Imports SystemMessageBus, MessagePassing, Message, ProcessId, KernelError, etc.
    // ProcessId is imported at the top of ipc.rs as `use crate::kernel::process::ProcessId;`
    // KernelError is imported at the top of ipc.rs as `use crate::kernel::KernelError;`

    // Helper to create a dummy ProcessId for testing.
    fn pid(id: u64) -> ProcessId { id }

    #[test]
    fn test_smb_new() {
        let bus = SystemMessageBus::new();
        assert!(bus.queues.is_empty(), "New bus should have no queues");
    }

    #[test]
    fn test_smb_send_and_receive_single_message() {
        let mut bus = SystemMessageBus::new();
        let sender_pid = pid(1);
        let receiver_pid = pid(2);
        let payload = vec![1, 2, 3];
        let message = Message::new(sender_pid, receiver_pid, payload.clone());

        assert!(bus.send_message(message.clone()).is_ok());

        // Check internal queue state (optional, but good for this level of testing)
        assert!(bus.queues.contains_key(&receiver_pid));
        assert_eq!(bus.queues.get(&receiver_pid).unwrap().len(), 1);

        let received_message_result = bus.receive_message(receiver_pid);
        assert!(received_message_result.is_ok());
        let received_message = received_message_result.unwrap();

        assert_eq!(received_message.id, message.id);
        assert_eq!(received_message.sender_pid, sender_pid);
        assert_eq!(received_message.receiver_pid, receiver_pid);
        assert_eq!(received_message.payload, payload);
        assert!(bus.queues.get(&receiver_pid).unwrap().is_empty());
    }

    #[test]
    fn test_smb_try_receive_message_empty_queue() {
        let mut bus = SystemMessageBus::new();
        let receiver_pid = pid(1);
        let result = bus.try_receive_message(receiver_pid);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none(), "try_receive on empty/non-existent queue should be None");
    }

    #[test]
    fn test_smb_receive_message_empty_queue_error() {
        let mut bus = SystemMessageBus::new();
        let receiver_pid = pid(1);
        let result = bus.receive_message(receiver_pid);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), KernelError::NotFound);
    }

    #[test]
    fn test_smb_send_creates_queue() {
        let mut bus = SystemMessageBus::new();
        let sender_pid = pid(1);
        let receiver_pid = pid(2);
        assert!(!bus.queues.contains_key(&receiver_pid), "Queue should not exist before send");
        let message = Message::new(sender_pid, receiver_pid, vec![1]);
        bus.send_message(message).unwrap();
        assert!(bus.queues.contains_key(&receiver_pid), "Queue should exist after send");
        assert_eq!(bus.queues.get(&receiver_pid).unwrap().len(), 1);
    }

    #[test]
    fn test_smb_fifo_order() {
        let mut bus = SystemMessageBus::new();
        let s_pid = pid(1);
        let r_pid = pid(2);
        let msg1 = Message::new(s_pid, r_pid, vec![1]);
        let msg2 = Message::new(s_pid, r_pid, vec![2]);
        let msg1_id = msg1.id;
        let msg2_id = msg2.id;

        bus.send_message(msg1).unwrap();
        bus.send_message(msg2).unwrap();

        let recv_msg1 = bus.receive_message(r_pid).unwrap();
        assert_eq!(recv_msg1.id, msg1_id);
        assert_eq!(recv_msg1.payload, vec![1]);

        let recv_msg2 = bus.receive_message(r_pid).unwrap();
        assert_eq!(recv_msg2.id, msg2_id);
        assert_eq!(recv_msg2.payload, vec![2]);
    }

    #[test]
    fn test_smb_try_receive_message_success() {
        let mut bus = SystemMessageBus::new();
        let sender_pid = pid(1);
        let receiver_pid = pid(2);
        let payload = vec![10, 20];
        let message = Message::new(sender_pid, receiver_pid, payload.clone());
        let original_message_id = message.id;

        bus.send_message(message).unwrap();

        let received_result = bus.try_receive_message(receiver_pid);
        assert!(received_result.is_ok());
        assert!(received_result.as_ref().unwrap().is_some());
        let received_message = received_result.unwrap().unwrap();

        assert_eq!(received_message.id, original_message_id);
        assert_eq!(received_message.payload, payload);
    }
}

// Trait for Inter-Process Communication (IPC) operations.
pub trait MessagePassing {
    /// Sends a message to a target process.
    fn send_message(&mut self, message: Message) -> KernelResult<()>;

    /// Receives a message for the given process ID.
    /// This version might block if no message is available, or return a specific error.
    /// For this phase, it will return Err(KernelError::NotFound) if no message.
    fn receive_message(&mut self, receiver_pid: ProcessId) -> KernelResult<Message>;

    /// Attempts to receive a message for the given process ID without blocking.
    /// Returns `Ok(None)` if no message is currently available.
    fn try_receive_message(&mut self, receiver_pid: ProcessId) -> KernelResult<Option<Message>>;
}

// SystemMessageBus struct and its impls will be added in the next step.

// Definition of the SystemMessageBus
#[derive(Debug, Default)] // Default will create an empty bus
pub struct SystemMessageBus {
    // Each process has its own queue of incoming messages.
    queues: HashMap<ProcessId, VecDeque<Message>>,
    // For true blocking, a wait_list might be needed:
    // wait_list: HashMap<ProcessId, Vec<ThreadId>>, // Key: ProcessId waiting for a message, Value: List of its threads that are blocked
}

impl SystemMessageBus {
    pub fn new() -> Self {
        Self::default() // Initializes queues (and wait_list if added) to empty HashMaps
    }
}

// Implementation of the MessagePassing trait for SystemMessageBus
impl MessagePassing for SystemMessageBus {
    fn send_message(&mut self, message: Message) -> KernelResult<()> {
        let receiver_queue = self.queues.entry(message.receiver_pid).or_insert_with(VecDeque::new);
        receiver_queue.push_back(message);

        // Conceptual: If processes/threads were waiting, notify them here.
        // e.g., if self.wait_list.contains_key(&message.receiver_pid) { /* wake up logic */ }
        Ok(())
    }

    fn receive_message(&mut self, receiver_pid: ProcessId) -> KernelResult<Message> {
        if let Some(receiver_queue) = self.queues.get_mut(&receiver_pid) {
            if let Some(message) = receiver_queue.pop_front() {
                Ok(message)
            } else {
                // Queue is empty
                Err(KernelError::NotFound) // Or a more specific NoMessageError if defined
            }
        } else {
            // No queue for this process (it hasn't received any messages yet)
            Err(KernelError::NotFound) // Or NoMessageError
        }
    }

    fn try_receive_message(&mut self, receiver_pid: ProcessId) -> KernelResult<Option<Message>> {
        if let Some(receiver_queue) = self.queues.get_mut(&receiver_pid) {
            if let Some(message) = receiver_queue.pop_front() {
                Ok(Some(message))
            } else {
                // Queue is empty
                Ok(None)
            }
        } else {
            // No queue for this process
            Ok(None)
        }
    }
}
