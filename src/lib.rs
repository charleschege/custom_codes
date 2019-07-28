#![deny(missing_docs)]

//! This are Custom Enum codes for memorable and uniform response codes.
//! Enums are cheaper to compair than strings thereby guaranteeing efficiency.
//! 
//! ```
//! 
//! use custom_codes::{DbOps, CustomBool}; 
//! fn dbop () -> DbOps { 
//!   if db_insert() == true { 
//!       DbOps::Inserted 
//!   }else if db_insert() == false { 
//!       DbOps::Skipped 
//!   }else { 
//!       DbOps::ConnRefused 
//!   } 
//! }
//! 
//! ```
//! 
//!

pub use custom_codes::{
    Outcome,
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
    ActivityInit,
    SecHardware
};

mod custom_codes;