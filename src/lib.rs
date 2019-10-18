#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(missing_doc_code_examples)]
//! This are Custom Enum codes for memorable and uniform response codes.
//! Enums are cheaper to compair than strings thereby guaranteeing efficiency.
//! 

/// re-exports
pub use crate::custom_codes::{
    Outcome,
    AccessStatus,
    DbOps, 
    CustomBool, 
    ExecCommand, 
    Cli,
    Networking, 
    Subscription, 
    FileOps, 
    DateTimeOp,
    ActivityStatus,
    Compression,
    SecOps,
    HardwareResources,
    ActivityToggle,
    SecHardware,
    AuthState,
    TempLock,
};

mod custom_codes;