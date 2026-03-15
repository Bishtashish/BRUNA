// bruna_os/src/kernel/ipc.rs
use crate::kernel::process::ProcessId;
use crate::kernel::KernelResult;
use crate::kernel::KernelError;
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::{HashMap, VecDeque};

// Static counter for generating unique MessageIds globally
static NEXT_MESSAGE_ID: AtomicU64 = AtomicU64::new(1);

pub type MessageId = u64;

// Function to generate a new unique MessageId
pub fn generate_mid() -> MessageId {
    NEXT_MESSAGE_ID.fetch_add(1, Ordering::Relaxed)
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: MessageId,
    pub sender_pid: ProcessId,
    pub receiver_pid: ProcessId,
    pub payload: Vec<u8>,
}

impl Message {
    pub fn new(sender_pid: ProcessId, receiver_pid: ProcessId, payload: Vec<u8>) -> Self {
        Message {
            id: generate_mid(),
            sender_pid,
            receiver_pid,
            payload,
        }
    }
}

pub trait MessagePassing {
    fn send_message(&mut self, message: Message) -> KernelResult<()>;
    fn receive_message(&mut self, receiver_pid: ProcessId) -> KernelResult<Message>;
    fn try_receive_message(&mut self, receiver_pid: ProcessId) -> KernelResult<Option<Message>>;
}

#[derive(Debug, Default)]
pub struct SystemMessageBus {
    queues: HashMap<ProcessId, VecDeque<Message>>,
}

impl SystemMessageBus {
    pub fn new() -> Self {
        Self::default()
    }
}

impl MessagePassing for SystemMessageBus {
    fn send_message(&mut self, message: Message) -> KernelResult<()> {
        let receiver_queue = self.queues.entry(message.receiver_pid).or_insert_with(VecDeque::new);
        receiver_queue.push_back(message);
        Ok(())
    }

    fn receive_message(&mut self, receiver_pid: ProcessId) -> KernelResult<Message> {
        if let Some(receiver_queue) = self.queues.get_mut(&receiver_pid) {
            if let Some(message) = receiver_queue.pop_front() {
                Ok(message)
            } else {
                Err(KernelError::NotFound)
            }
        } else {
            Err(KernelError::NotFound)
        }
    }

    fn try_receive_message(&mut self, receiver_pid: ProcessId) -> KernelResult<Option<Message>> {
        if let Some(receiver_queue) = self.queues.get_mut(&receiver_pid) {
            if let Some(message) = receiver_queue.pop_front() {
                Ok(Some(message))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pid(id: u64) -> ProcessId { id }

    #[test]
    fn test_smb_new() {
        let bus = SystemMessageBus::new();
        assert!(bus.queues.is_empty());
    }

    #[test]
    fn test_smb_send_and_receive_single_message() {
        let mut bus = SystemMessageBus::new();
        let sender_pid = pid(1);
        let receiver_pid = pid(2);
        let payload = vec![1, 2, 3];
        let message = Message::new(sender_pid, receiver_pid, payload.clone());

        assert!(bus.send_message(message.clone()).is_ok());

        let received_message = bus.receive_message(receiver_pid).unwrap();
        assert_eq!(received_message.sender_pid, sender_pid);
        assert_eq!(received_message.payload, payload);
    }

    #[test]
    fn test_smb_try_receive_message_empty_queue() {
        let mut bus = SystemMessageBus::new();
        let receiver_pid = pid(1);
        let result = bus.try_receive_message(receiver_pid);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_smb_receive_message_empty_queue_error() {
        let mut bus = SystemMessageBus::new();
        let receiver_pid = pid(1);
        let result = bus.receive_message(receiver_pid);
        assert!(result.is_err());
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

        assert_eq!(bus.receive_message(r_pid).unwrap().id, msg1_id);
        assert_eq!(bus.receive_message(r_pid).unwrap().id, msg2_id);
    }
}
