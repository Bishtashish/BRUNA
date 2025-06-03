// bruna_os/src/kernel/scheduler.rs
use crate::kernel::thread::ThreadId;
use crate::kernel::KernelResult;
use std::collections::VecDeque; // For the ready queue

// New/Refined Scheduler Trait Definition:
pub trait Scheduler {
    /// Adds a thread to the scheduler's ready queue.
    /// This method is typically called when a thread becomes ready to run.
    fn add_thread(&mut self, tid: ThreadId) -> KernelResult<()>;

    /// Removes a thread from the scheduler's ready queue.
    /// This is called when a thread is terminated, blocks, or is no longer ready.
    fn remove_thread(&mut self, tid: ThreadId) -> KernelResult<()>;

    /// Selects the next thread to run according to the scheduling policy.
    /// It does not perform the context switch itself.
    /// Returns `None` if no threads are currently ready to run.
    fn schedule_next(&mut self) -> Option<ThreadId>;

    /// Marks a thread as ready to be scheduled.
    /// For simple schedulers, this might be an alias for `add_thread`.
    /// Could involve more complex logic in advanced schedulers (e.g., priority updates).
    fn mark_thread_ready(&mut self, tid: ThreadId) -> KernelResult<()> {
        // Default implementation often calls add_thread
        self.add_thread(tid)
    }

    /// Marks a thread as blocked and not ready to be scheduled.
    /// For simple schedulers, this might be an alias for `remove_thread`.
    fn mark_thread_blocked(&mut self, tid: ThreadId) -> KernelResult<()> {
        // Default implementation often calls remove_thread
        self.remove_thread(tid)
    }

    // fn set_priority(tid: ThreadId, priority: u8) -> KernelResult<()>; // Example for future
}

// No other structs needed for this step.
// RoundRobinScheduler will be defined in the next step.

// Definition of the RoundRobinScheduler
#[derive(Debug, Default)] // Default will create an empty queue
pub struct RoundRobinScheduler {
    ready_queue: VecDeque<ThreadId>,
}

impl RoundRobinScheduler {
    pub fn new() -> Self {
        Self::default() // Or RoundRobinScheduler { ready_queue: VecDeque::new() }
    }
}

// Implementation of the Scheduler trait for RoundRobinScheduler
impl Scheduler for RoundRobinScheduler {
    fn add_thread(&mut self, tid: ThreadId) -> KernelResult<()> {
        if !self.ready_queue.contains(&tid) {
            self.ready_queue.push_back(tid);
        }
        // else: Thread already in ready queue, which is fine, no error needed.
        // Or, return Err(KernelError::AlreadyExists) if strict. For now, idempotent is okay.
        Ok(())
    }

    fn remove_thread(&mut self, tid: ThreadId) -> KernelResult<()> {
        // Retain all elements that are NOT equal to tid.
        // This is one way to remove all occurrences if any.
        // If TIDs are guaranteed unique in the queue, a simple find and remove is also possible.
        if let Some(pos) = self.ready_queue.iter().position(|&x| x == tid) {
            self.ready_queue.remove(pos); // Removes the first occurrence
            Ok(())
        } else {
            // Thread not found in ready queue. This might not be an error in some contexts
            // (e.g. trying to remove a thread that already terminated and was removed).
            // For strictness, one might return Err(KernelError::NotFound)
            // For now, let's consider it not an error to try to remove a non-queued thread.
            Ok(())
        }
    }

    fn schedule_next(&mut self) -> Option<ThreadId> {
        if let Some(tid) = self.ready_queue.pop_front() {
            // In a real preemptive scheduler, we'd only re-add if the thread is still runnable
            // and hasn't yielded or blocked. For basic round-robin, always re-add.
            self.ready_queue.push_back(tid); // Move to the back of the queue
            Some(tid)
        } else {
            None // No threads in the ready queue
        }
    }

    // mark_thread_ready and mark_thread_blocked will use the default implementations
    // provided in the Scheduler trait, which call add_thread and remove_thread respectively.
    // If specific behavior is needed for RoundRobinScheduler for these, they can be overridden here.
}

#[cfg(test)]
mod tests {
    use super::*; // Imports RoundRobinScheduler, Scheduler trait, ThreadId, KernelResult

    #[test]
    fn test_rr_scheduler_new() {
        let scheduler = RoundRobinScheduler::new();
        assert!(scheduler.ready_queue.is_empty(), "New scheduler's ready queue should be empty");
    }

    #[test]
    fn test_rr_add_single_thread() {
        let mut scheduler = RoundRobinScheduler::new();
        let tid1: ThreadId = 1;
        assert!(scheduler.add_thread(tid1).is_ok());
        assert_eq!(scheduler.ready_queue.len(), 1);
        assert_eq!(scheduler.ready_queue.front(), Some(&tid1));
    }

    #[test]
    fn test_rr_add_multiple_threads() {
        let mut scheduler = RoundRobinScheduler::new();
        let tid1: ThreadId = 1;
        let tid2: ThreadId = 2;
        scheduler.add_thread(tid1).unwrap();
        scheduler.add_thread(tid2).unwrap();
        assert_eq!(scheduler.ready_queue.len(), 2);
        assert_eq!(scheduler.ready_queue.get(0), Some(&tid1));
        assert_eq!(scheduler.ready_queue.get(1), Some(&tid2));
    }

    #[test]
    fn test_rr_add_duplicate_thread_is_idempotent() {
        let mut scheduler = RoundRobinScheduler::new();
        let tid1: ThreadId = 1;
        scheduler.add_thread(tid1).unwrap();
        scheduler.add_thread(tid1).unwrap(); // Add again
        assert_eq!(scheduler.ready_queue.len(), 1, "Duplicate add should be idempotent");
    }

    #[test]
    fn test_rr_schedule_next_empty() {
        let mut scheduler = RoundRobinScheduler::new();
        assert_eq!(scheduler.schedule_next(), None, "schedule_next on empty queue should return None");
    }

    #[test]
    fn test_rr_schedule_next_single_thread_cycles() {
        let mut scheduler = RoundRobinScheduler::new();
        let tid1: ThreadId = 1;
        scheduler.add_thread(tid1).unwrap();

        assert_eq!(scheduler.schedule_next(), Some(tid1), "First schedule_next should return tid1");
        assert_eq!(scheduler.ready_queue.len(), 1, "Queue should still have 1 thread");
        assert_eq!(scheduler.ready_queue.front(), Some(&tid1), "tid1 should be back in queue (at front as it's the only one)");

        assert_eq!(scheduler.schedule_next(), Some(tid1), "Second schedule_next should also return tid1");
    }

    #[test]
    fn test_rr_schedule_next_multiple_threads_round_robin() {
        let mut scheduler = RoundRobinScheduler::new();
        let tid1: ThreadId = 1;
        let tid2: ThreadId = 2;
        let tid3: ThreadId = 3;
        scheduler.add_thread(tid1).unwrap();
        scheduler.add_thread(tid2).unwrap();
        scheduler.add_thread(tid3).unwrap();

        assert_eq!(scheduler.schedule_next(), Some(tid1)); // t1 scheduled, moved to back: [t2, t3, t1]
        assert_eq!(scheduler.ready_queue.back(), Some(&tid1));
        assert_eq!(scheduler.schedule_next(), Some(tid2)); // t2 scheduled, moved to back: [t3, t1, t2]
        assert_eq!(scheduler.ready_queue.back(), Some(&tid2));
        assert_eq!(scheduler.schedule_next(), Some(tid3)); // t3 scheduled, moved to back: [t1, t2, t3]
        assert_eq!(scheduler.ready_queue.back(), Some(&tid3));
        assert_eq!(scheduler.schedule_next(), Some(tid1)); // Loop back to t1: [t2, t3, t1]
        assert_eq!(scheduler.ready_queue.back(), Some(&tid1));
    }

    #[test]
    fn test_rr_remove_existing_thread() {
        let mut scheduler = RoundRobinScheduler::new();
        let tid1: ThreadId = 1;
        let tid2: ThreadId = 2;
        scheduler.add_thread(tid1).unwrap();
        scheduler.add_thread(tid2).unwrap();

        assert!(scheduler.remove_thread(tid1).is_ok());
        assert_eq!(scheduler.ready_queue.len(), 1);
        assert_eq!(scheduler.ready_queue.front(), Some(&tid2));
        assert!(!scheduler.ready_queue.contains(&tid1));
    }

    #[test]
    fn test_rr_remove_non_existing_thread_is_idempotent() {
        let mut scheduler = RoundRobinScheduler::new();
        let tid1: ThreadId = 1;
        scheduler.add_thread(tid1).unwrap();

        let non_existent_tid: ThreadId = 99;
        assert!(scheduler.remove_thread(non_existent_tid).is_ok(), "Removing non-existent thread should be Ok");
        assert_eq!(scheduler.ready_queue.len(), 1, "Queue should be unchanged");
    }

    #[test]
    fn test_rr_remove_from_empty_queue() {
        let mut scheduler = RoundRobinScheduler::new();
        let non_existent_tid: ThreadId = 99;
        assert!(scheduler.remove_thread(non_existent_tid).is_ok());
        assert!(scheduler.ready_queue.is_empty());
    }

    #[test]
    fn test_rr_mark_thread_ready_blocked_defaults() {
        let mut scheduler = RoundRobinScheduler::new();
        let tid1: ThreadId = 1;

        // mark_thread_ready uses add_thread
        scheduler.mark_thread_ready(tid1).unwrap();
        assert_eq!(scheduler.ready_queue.len(), 1);
        assert!(scheduler.ready_queue.contains(&tid1));

        // mark_thread_blocked uses remove_thread
        scheduler.mark_thread_blocked(tid1).unwrap();
        assert!(scheduler.ready_queue.is_empty());
    }
}
