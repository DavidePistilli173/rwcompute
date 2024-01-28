//! Error types.

use std::{error::Error, fmt};

/// Possible errors during context initialisation.
#[derive(Debug, Copy, Clone)]
pub enum ContextCreationError {
    /// Error while retrieving a compatible compute device (graphics card or other).
    NoPhysicalComputeDevice,
    /// Error while creating a logical rendering device or the command queue.
    DeviceOrQueueCreation,
}

impl Error for ContextCreationError {}

impl fmt::Display for ContextCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::NoPhysicalComputeDevice => {
                write!(f, "Failed to get a compatible physical compute device.")
            }
            Self::DeviceOrQueueCreation => write!(
                f,
                "Failed to create a logical compute device or a command queue."
            ),
        }
    }
}
