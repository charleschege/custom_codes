#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(missing_doc_code_examples)]
//! This are Custom Enum codes for memorable and uniform response codes.
//! Enums are cheaper to compair than strings thereby guaranteeing efficiency.
//!

/// re-exports
pub use crate::custom_codes::{
    AccessStatus, ActivityStatus, ActivityToggle, Cli, Compression, CustomBool, DateTimeOp, DbOps,
    ExecCommand, FileOps, HardwareResources, Networking, Outcome, SecHardware, SecOps,
    Subscription,
};

mod custom_codes;
