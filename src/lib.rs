#![deny(missing_docs)]

//! This are Custom Enum codes for memorable and uniform response codes.
//! Enums are cheaper to compair than strings thereby guaranteeing efficiency.
//! 

pub use custom_codes::{
    DbOps, 
    CustomBool, 
    ExecCommand, 
    Networking, 
    Subscription, 
    FileOps, 
    DateTimeOp,
    ActivityStatus,
    Compression,
    SecOps,
    HardwareResources,
    ActivityInit,
    SecHardware
};

mod custom_codes;