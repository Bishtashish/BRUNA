// bruna_os/src/kernel/mod.rs
pub mod process;
pub mod thread;
pub mod ipc;
pub mod scheduler; // Added for basic scheduling concepts
pub mod memory;    // Added for basic memory management concepts
pub mod registry;
pub mod discovery;

// Placeholder for a generic Kernel Error type
#[derive(Debug, PartialEq, Eq)] // Added PartialEq, Eq
pub enum KernelError {
    NotFound,
    Permissions,
    MemoryNotAvailable,
    IPCError(String),
    FeatureNotImplemented,
    Other(String),      // Ensure this variant is present
    AlreadyExists,      // Add this useful variant
    InvalidState(String), // Potentially useful for state-related errors
}

pub type KernelResult<T> = Result<T, KernelError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Importance {
    Optional = 0,
    Essential = 1,
    Critical = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Internal,
    Serial,
    Gpio,
    I2c,
    Spi,
    Radio,
    Network,
}

// A top-level trait for the kernel itself, if needed later
// pub trait Kernel {
//     fn version(&self) -> &'static str;
// }
