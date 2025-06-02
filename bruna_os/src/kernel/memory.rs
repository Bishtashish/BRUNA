// bruna_os/src/kernel/memory.rs
use super::KernelResult;
use super::process::ProcessId;

pub type Address = usize;
pub type Size = usize;

// Represents a block of memory allocated to a process (very abstract)
pub struct MemoryRegion {
    pub start_address: Address,
    pub size: Size,
    pub process_id: ProcessId,
    // pub permissions: MemoryPermissions,
}

// pub struct MemoryPermissions {
//     read: bool,
//     write: bool,
//     execute: bool,
// }

pub trait MemoryManagement {
    fn allocate(pid: ProcessId, size: Size) -> KernelResult<Address>;
    fn deallocate(pid: ProcessId, address: Address) -> KernelResult<()>;
    // fn map_memory(...) -> KernelResult<()>;
    // fn get_free_memory() -> Size;
}
