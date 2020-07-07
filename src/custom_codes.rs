use serde::{Deserialize, Serialize};

/// Give the Outcome of an operation
/// ### Examples
/// ```
/// # use custom_codes::Outcome;
/// let foo = Outcome::Success;
/// assert_eq!(foo, Outcome::Success);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Outcome {
    ///Result of Operation completed successfully
    Success,
    /// Result of Operation produced an error
    Failure,
    /// The result of the operation was forwarded to another operation
    Forward,
}
/// Give the Generic Outcome of an operation with custom result as a generic
/// ### Examples
/// ```
/// # use custom_codes::GenericOutcome;
/// let foo = GenericOutcome::Success("Foo");
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum GenericOutcome<Reason> {
    ///Result of Operation completed successfully
    Success(Reason),
    /// Result of Operation produced an error
    Failure(Reason),
    /// The result of the operation was forwarded to another operation
    Forward(Reason),
}
/// Access status of token or access
/// ### Examples
/// ```
/// # use custom_codes::AccessStatus;
/// let foo = AccessStatus::Granted;
/// assert_eq!(foo, AccessStatus::Granted);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum AccessStatus {
    /// Process or User has been given a lease
    Granted,
    /// Token has been discarded
    Revoked,
    /// Access to token or session has expired
    Expired,
    /// Access has been denied
    Denied,
    /// The access token has been rejected because it is not authentic/genuine
    Rejected,
    /// Operation is being executed
    InProgress,
    /// Random Authentication Code (RAC) Token is genuine/authentic and therefore has been accepted
    AccpetedRAC,
    ///  Random Authentication Code (RAC) Token is not genuine/authentic and therefore has been rejected
    RejectedRAC,
    /// Status of an operation is not initialized
    Unspecified,
}

/// Creates a custom `boolean` value with more features than a Rustlang boolean which has only `true` or `false`
/// ### Examples
/// ```
/// # use custom_codes::CustomBool;
/// let foo = CustomBool::Unspecified;
/// assert_eq!(foo, CustomBool::Unspecified);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum CustomBool {
    /// Similar to boolean true
    True,
    /// Similar to boolean false
    False,
    /// Status of an operation is not initialized
    Unspecified,
}

/// Custom Response Codes Using Enums For Efficient Comparison
/// ### Examples
/// ```
/// # use custom_codes::DbOps;
/// let foo = DbOps::Inserted;
/// assert_eq!(foo, DbOps::Inserted);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum DbOps {
    /// A Repository has been created
    RepoCreated,
    /// A Repository has been initialized by loading all contents of the repo
    RepoInitialized,
    /// The repository trying to be created already exists
    RepoAlreadyExists,
    /// An empty Repository with no databases
    RepoEmpty,
    /// An repository is not available on disk
    RepoNotFound,
    /// A Repository has been deleted
    RepoDropped,
    /// A checksum of the database shows a database is consistent
    DbIntegrityConsistent,
    /// A checksum fo the database shows the database is not consistent
    DbIntegrityCorrupted,
    /// A checksum of the document is consistent
    DocumentIntegrityConsistent,
    /// A checksum fo the document is not consistent
    DocumentIntegrityCorrupted,
    /// Database has been created
    DbCreated,
    /// Database has been Found after a search
    DbFound,
    /// Database being inserted already exists in the repo
    DbAlreadyExists,
    /// Database does not exist
    DbNotFound,
    /// Database is empty
    DbEmpty,
    /// A list containing Databases
    DbList(Vec<String>),
    /// A Database has neen modified
    DbModified,
    /// A database has been removed
    DbDropped,
    /// Document has been created
    DocumentCreated,
    /// Document has been inserted
    DocumentInserted,
    /// Document has been Found after a search
    DocumentFound,
    /// Document is empty
    DocumentEmpty,
    /// List of Documents
    DocumentList(Vec<String>),
    /// Document being inserted already exists in the DB
    DocumentAlreadyExists,
    /// Document does not exist
    DocumentNotFound,
    /// A Document has neen modified
    DocumentModified,
    /// Document has been removed
    DocumentDropped,
    /// Field has been created
    FieldCreated,
    /// Field has been inserted
    FieldInserted,
    /// Field has been Found after a search
    FieldFound,
    /// Field being inserted already exists in the DB
    FieldAlreadyExists,
    /// Field does not exist
    FieldNotFound,
    /// Contents of a field in heap allocated bytes
    FieldContents(Vec<u8>),
    /// List of fields in a document
    FieldList(Vec<Vec<u8>>),
    /// A Field has neen modified
    FieldModified,
    /// Field has been removed
    FieldDropped,
    /// A `Write` to create a database is successfull
    Created,
    /// A `Write` to a commit log was successful
    Commited,
    /// A `Write` to a commit log was unsuccessful
    UnCommited,
    /// A `Write` is successful
    Inserted,
    /// A `Change` is successful
    Changed,
    /// A `Change` is unsuccessful
    Unchanged,
    /// A `Command` is skipped since the document does not exist
    Skipped,
    /// Document already exists
    AlreadyExists,
    /// Document has been updated
    Modified,
    /// A document value has been `Swapped` replacing it with the new value provided
    /// Especially useful for Key/Value stores
    Swapped,
    /// An Document has been deleted
    Deleted,
    /// Key Found in KV Store/Database, Field or Document depending on query
    KeyFound,
    /// Key Not Found in Database, Field or Document depending on query
    KeyNotFound,
    /// A Command or Query Has Not Been Executed
    NotExecuted,
    /// Two or more queries are not supposed to be used command eg. using `get` and `list` together
    QueryConflictError,
    /// Database is empty
    Empty,
    /// The command has been added to a queue
    Queued,
    /// Command has been removed from queue
    DeQueued,
    /// Log compaction successful
    CompactionTrue,
    /// Log compaction unsuccessful
    CompactionFalse,
    /// Connection to a database is not available
    ConnRefused,
    /// Port is in use
    PortAddrInUse,
    /// Driver to the database has an error
    DriverError,
    /// An error occured when trying to execute a command at database level
    RuntimeError,
    /// Encountered Errors When Trying to connect to a database
    EncounteredErrors(String),
    /// No permissions to complete the I/O operation
    PermissionDenied,
    /// A cluster in online and ready to receive commands
    ClusterOnline,
    /// A cluster is offline
    ClusterOffline,
    /// Cluster details synced across all nodes
    ClusterSynced,
    /// Added a node to the cluster
    ClusterNodeAdded,
    /// Updated configuration to the cluster
    ConfigUpdated,
    /// A node has been permanently removed
    ClusterNodeDropped,
    /// The details have been inserted to the specified logs
    Logged,
    /// The details have not been logged
    NotLogged,
    /// The Operation was interrupted and can be resumed
    Interrupted,
    /// Status of an operation is not initialized
    Unspecified,
}

/// Command Operations for execution
/// ### Examples
/// ```
/// # use custom_codes::ExecCommand;
/// let foo = ExecCommand::Queued;
/// assert_eq!(foo, ExecCommand::Queued);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ExecCommand {
    /// Command Operation added to Queue
    Queued,
    /// Command removed from queue
    DeQueued,
    /// Command in progress
    InProgress,
    /// Command paused
    Paused,
    /// Command suspended but not killed
    Frozen,
    /// Command killed and removed from execution stack
    Killed,
    /// Command has finished execution
    Executed,
    /// The Operation was interrupted and can be resumed
    Interrupted,
    /// Status of an operation is not initialized
    Unspecified,
}

/// File operations
/// ### Examples
/// ```
/// # use custom_codes::FileOps;
/// let foo = FileOps::DirAccessDenied;
/// assert_eq!(foo, FileOps::DirAccessDenied);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FileOps {
    /// Directory Created
    DirCreated,
    /// Directory Access Denied
    DirAccessDenied,
    /// Directory is read only
    DirIsReadOnly,
    /// Directory is append only
    DirectoryAppendOnly,
    /// File is read only
    FileIsReadOnly,
    /// File is append only
    FileIsAppendOnly,
    /// Delete operation denied due to permissions
    DeletionDenied,
    /// Deletion could not be done as another operation is reading the file
    DeletionToBeDone,
    /// Read has been completed especially useful in `async` operations
    ReadDone,
    /// Write operation has been competed
    WriteDone,
    /// The file or directory already exists
    AlreadyExists,
    /// Update operation completed
    UpdateDone,
    /// Deletion operation completed
    DeletionDone,
    /// Read is in progress
    ReadInProgress,
    /// Write is in progress
    WriteInProgess,
    /// Update is in progress
    UpdateInProgress,
    /// File has been deleted
    DeletionInProgress,
    /// Directory has been opened
    DirOpened,
    /// Directory is no longer open
    DirClosed,
    /// Directory has been modified
    DirModified,
    /// File has been opened
    OpenedFile,
    /// File has been closed
    ClosedFile,
    /// Unable To Open File
    OpenError,
    /// The operation comleted with a given error
    EncounteredErrors(String),
    /// Creation of the file is denied
    CreateDenied,
    /// Read acess to file is denied
    ReadDenied,
    /// Update to the file is denied
    UpdateDenied,
    /// Writing to the file has been denied
    WriteDenied,
    /// Appending to the file has been denied
    AppendDenied,
    /// Metadata has been added to file or directory
    MetadataAdded,
    /// File or directory does not contain Metadata
    MetadataNotAvailable,
    /// metadata changed
    MetadataChanged,
    /// Netadata has been Deleted
    MetadataDeleted,
    /// Open a file. any open can read contents
    OpenTrue,
    /// Do not open file
    OpenFalse,
    /// Create a file if it does not exist
    CreateTrue,
    /// Do Not Create a file it does not exist
    CreateFalse,
    /// Write to a file
    WriteTrue,
    /// Do not write to file
    WriteFalse,
    /// Append to a file
    AppendTrue,
    /// The Operation was interrupted and can be resumed
    Interrupted,
    /// Delete a file if it exists,
    DeleteTrue,
}

/// Compression of files, bytes or directories
/// ### Examples
/// ```
/// # use custom_codes::Compression;
/// let foo = Compression::Done;
/// assert_eq!(foo, Compression::Done);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Compression {
    /// Compression done
    Done,
    /// Compression in progress
    InProgress,
    /// Compression already done
    AlreadyCompressed,
    /// Compression is not possible
    Impossible,
    /// A process is writing to the file being compressed
    ToBeDone,
    /// Streaming and compressing at the same time
    StreamCompress,
    /// The Operation was interrupted and can be resumed
    Interrupted,
    /// Status of an operation is not initialized
    Unspecified,
}

/// Status of a subscription that happens over a timespec
/// ### Examples
/// ```
/// # use custom_codes::Subscription;
/// let foo = Subscription::Subscribed;
/// assert_eq!(foo, Subscription::Subscribed);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Subscription {
    /// Subscription is active
    Subscribed,
    /// Subscription is inactive
    Unsubscribed,
    /// Subscription is not yet activated
    NotActivated,
    /// Status of an operation is not initialized
    Unspecified,
}

/// Status of an activity
/// ### Examples
/// ```
/// # use custom_codes::ActivityStatus;
/// let foo = ActivityStatus::Activated;
/// assert_eq!(foo, ActivityStatus::Activated);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ActivityStatus {
    /// Status of an operation has been started
    Activated,
    /// Status of an operation has been stopped
    Deactivated,
    /// Status of an operation is not initialized
    Unspecified,
}
/// Initialize an activity
/// ### Examples
/// ```
/// # use custom_codes::ActivityToggle;
/// let foo = ActivityToggle::Activate;
/// assert_eq!(foo, ActivityToggle::Activate);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ActivityToggle {
    /// Activate an operation
    Activate,
    /// Deactivate an operation
    Deactivate,
    /// Status of an operation is not initialized
    Unspecified,
}
/// Command Line Options
/// ### Examples
/// ```
/// # use custom_codes::Cli;
/// let foo = Cli::ParseInProgress;
/// assert_eq!(foo, Cli::ParseInProgress);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Cli {
    /// Parsing
    ParseInProgress,
    /// Parsed Arguments
    ParseArgs,
    /// Parsing Argument field in progress
    ArgFieldParseInPorgress,
    /// Parsed Argument field
    ParsedArgField,
    /// Parsing Asynchronously
    ParseAsync,
    /// Parsing field Asynchronously
    ArgFieldAsyncParseInPorgress,
    /// Not enough commandline arguments
    NotEnoughArgs,
    /// Encountered parsing errors
    EncounteredErrors(String),
}
/// Date and time custom codes
/// ### Examples
/// ```
/// # use custom_codes::DateTimeOp;
/// let foo = DateTimeOp::DateCreated;
/// assert_eq!(foo, DateTimeOp::DateCreated);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum DateTimeOp {
    /// Date created
    DateCreated,
    /// Date deleted
    DateDeleted,
    /// Date Modified
    DateModified,
    /// Date period not specified
    DateUnspecified,
    /// Date created
    TimeCreated,
    /// Date deleted
    TimeDeleted,
    /// Date Modified
    TimeModified,
    /// Time period not specified
    TimeUnspecified,
    /// Both date and time not specied
    DateTimeUnspecified,
}

/// Security operations on encryption and keys
/// ### Examples
/// ```
/// # use custom_codes::SecOps;
/// let foo = SecOps::KeyCorrupted;
/// assert_eq!(foo, SecOps::KeyCorrupted);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum SecOps {
    /// Key could not be verified as it did not finish streaming
    KeyCorrupted,
    /// Key Authentic and approved
    KeyAuthentic,
    /// Key has been tampered with
    KeyInvalid,
    /// Timed Out while verification was in progress
    TimedOut,
    /// Possible Identity Forgery
    PossibleIdTheft,
    /// Data encrypted has been corrupted
    DataCorrupted,
    /// Data has been tampered with
    DataInvalid,
    /// Data has been encrypted
    Encrypted,
    /// Key generation successful,
    KeyGenSucceded,
    /// Key generation failed
    KeyGenFailed,
    /// Key has been deleted
    KeyDeleted,
    /// random data generated from Cryptographically Secure PRNG (CSPRNG)
    CryptoRandomGenerated,
    /// random data generated from Cryptographically Secure PRNG (CSPRNG) was not generated successfully
    CryptoRandomGenFailure,
    /// random data generated from Noncryptographic PRNG
    NonCryptoRandomGenerated,
    /// random data generated from Noncryptographic PRNG was not generated successfully
    NonCryptoRandomGenFailure,
    /// Message Authentication Code is authentic
    AuthenticMAC,
    /// Message Authentication Code is corrupted
    InvalidMAC,
    /// Random Authentication Code (RAC) Token is genuine/authentic
    ValidRAC,
    ///  Random Authentication Code (RAC) Token is not genuine/authentic
    InvalidRAC,
    /// Key Length is equal to the length needed by the cryptography algorithm
    KeyLengthSane,
    /// Key length too short
    KeyTooShort(KeyLength),
    /// Key Length is too long
    KeyTooLong(KeyLength),
}

impl std::fmt::Display for SecOps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecOps::KeyCorrupted => write!(f, "Key could not be verified as it did not finish streaming"),
            SecOps::KeyAuthentic => write!(f, "Key Authentic and approved"),
            SecOps::KeyInvalid => write!(f, "Key has been tampered with"),
            SecOps::TimedOut => write!(f, "Timed Out while verification was in progress"),
            SecOps::PossibleIdTheft => write!(f, "Possible Identity Forgery"),
            SecOps::DataCorrupted => write!(f, "Data encrypted has been corrupted"),
            SecOps::DataInvalid => write!(f, "Data has been tampered with"),
            SecOps::Encrypted => write!(f, "Data has been encrypted"),
            SecOps::KeyGenSucceded => write!(f, "Key generation successful"),
            SecOps::KeyGenFailed => write!(f, "Key generation failed"),
            SecOps::KeyDeleted => write!(f, "Key has been deleted"),
            SecOps::CryptoRandomGenerated => write!(f, "random data generated from Cryptographically Secure PRNG (CSPRNG)"),
            SecOps::CryptoRandomGenFailure => write!(f, "random data generated from Cryptographically Secure PRNG (CSPRNG) was not generated"),
            SecOps::NonCryptoRandomGenerated => write!(f, "random data generated from Noncryptographic PRNG"),
            SecOps::NonCryptoRandomGenFailure => write!(f, "random data generated from Noncryptographic PRNG was not generated"),
            SecOps::AuthenticMAC => write!(f, "Message Authentication Code is authentic"),
            SecOps::InvalidMAC => write!(f, "Message Authentication Code is corrupted"),
            SecOps::ValidRAC => write!(f, "Random Authentication Code (RAC) Token is genuine/authentic"),
            SecOps::InvalidRAC => write!(f, "Random Authentication Code (RAC) Token is not genuine/authentic"),
            SecOps::KeyLengthSane => write!(f, "Key Length is equal to the length needed by the cryptography algorithm"),
            SecOps::KeyTooShort(_) => write!(f, "Key length too short"),
            SecOps::KeyTooLong(_) => write!(f, "Key Length is too long"),
        }
    }
}

impl std::error::Error for SecOps {
    fn description(&self) -> &str {
        match self {
            SecOps::KeyCorrupted => "Key could not be verified as it did not finish streaming",
            SecOps::KeyAuthentic => "Key Authentic and approved",
            SecOps::KeyInvalid => "Key has been tampered with",
            SecOps::TimedOut => "Timed Out while verification was in progress",
            SecOps::PossibleIdTheft => "Possible Identity Forgery",
            SecOps::DataCorrupted => "Data encrypted has been corrupted",
            SecOps::DataInvalid => "Data has been tampered with",
            SecOps::Encrypted => "Data has been encrypted",
            SecOps::KeyGenSucceded => "Key generation successful",
            SecOps::KeyGenFailed => "Key generation failed",
            SecOps::KeyDeleted => "Key has been deleted",
            SecOps::CryptoRandomGenerated => "random data generated from Cryptographically Secure PRNG (CSPRNG)",
            SecOps::CryptoRandomGenFailure => "random data generated from Cryptographically Secure PRNG (CSPRNG) was not generated",
            SecOps::NonCryptoRandomGenerated => "random data generated from Noncryptographic PRNG",
            SecOps::NonCryptoRandomGenFailure => "random data generated from Noncryptographic PRNG was not generated",
            SecOps::AuthenticMAC => "Message Authentication Code is authentic",
            SecOps::InvalidMAC => "Message Authentication Code is corrupted",
            SecOps::ValidRAC => "Random Authentication Code (RAC) Token is genuine/authentic",
            SecOps::InvalidRAC => "Random Authentication Code (RAC) Token is not genuine/authentic",
            SecOps::KeyLengthSane => "Key Length is equal to the length needed by the cryptography algorithm",
            SecOps::KeyTooShort(_) => "Key length too short",
            SecOps::KeyTooLong(_) => "Key Length is too long",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            SecOps::KeyCorrupted => Some(&SecOps::KeyCorrupted),
            SecOps::KeyAuthentic => Some(&SecOps::KeyAuthentic),
            SecOps::KeyInvalid => Some(&SecOps::KeyInvalid),
            SecOps::TimedOut => Some(&SecOps::TimedOut),
            SecOps::PossibleIdTheft => Some(&SecOps::PossibleIdTheft),
            SecOps::DataCorrupted => Some(&SecOps::DataCorrupted),
            SecOps::DataInvalid => Some(&SecOps::DataInvalid),
            SecOps::Encrypted => Some(&SecOps::Encrypted),
            SecOps::KeyGenSucceded => Some(&SecOps::KeyGenSucceded),
            SecOps::KeyGenFailed => Some(&SecOps::KeyGenFailed),
            SecOps::KeyDeleted => Some(&SecOps::KeyDeleted),
            SecOps::CryptoRandomGenerated => Some(&SecOps::CryptoRandomGenerated),
            SecOps::CryptoRandomGenFailure => Some(&SecOps::CryptoRandomGenFailure),
            SecOps::NonCryptoRandomGenerated => Some(&SecOps::NonCryptoRandomGenFailure),
            SecOps::NonCryptoRandomGenFailure => Some(&SecOps::NonCryptoRandomGenFailure),
            SecOps::AuthenticMAC => Some(&SecOps::AuthenticMAC),
            SecOps::InvalidMAC => Some(&SecOps::InvalidMAC),
            SecOps::ValidRAC => Some(&SecOps::ValidRAC),
            SecOps::InvalidRAC => Some(&SecOps::InvalidRAC),
            SecOps::KeyLengthSane => Some(&SecOps::KeyLengthSane),
            SecOps::KeyTooShort(ref length) => length.source(),
            SecOps::KeyTooLong(ref length) => length.source(),
        }
    }
}

/// The length of a key in bytes needed by a cryptography algorithm
/// ### Examples
/// ```
/// # use custom_codes::KeyLength;
/// let foo = KeyLength::Bytes32;
/// assert_eq!(foo, KeyLength::Bytes32);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum KeyLength {
    /// A Key length of 8 bytes
    Bytes8,
    /// A Key length of 16 bytes
    Bytes16,
    /// A Key length of 24 bytes
    Bytes24,
    /// A Key length of 32 bytes
    Bytes32,
    /// A Key length of 64 bytes
    Bytes64,
    /// A Key length of 128 bytes
    Bytes128,
    /// A Key length of 256 bytes
    Bytes256,
    /// A Key length of 512 bytes
    Bytes512,
    /// A Key length of 1024 bytes
    Bytes1024,
    /// A Key length of 2048 bytes
    Bytes2048,
    /// A Key length of 4096 bytes
    Bytes4096,
}

impl std::fmt::Display for KeyLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyLength::Bytes8 => write!(f, "KeyLength::Bytes8"),
            KeyLength::Bytes16 => write!(f, "KeyLength::Bytes16"),
            KeyLength::Bytes24 => write!(f, "KeyLength::Bytes24"),
            KeyLength::Bytes32 => write!(f, "KeyLength::Bytes32"),
            KeyLength::Bytes64 => write!(f, "KeyLength::Bytes64"),
            KeyLength::Bytes128 => write!(f, "KeyLength::Bytes128"),
            KeyLength::Bytes256 => write!(f, "KeyLength::Bytes256"),
            KeyLength::Bytes512 => write!(f, "KeyLength::Bytes512"),
            KeyLength::Bytes1024 => write!(f, "KeyLength::Bytes1024"),
            KeyLength::Bytes2048 => write!(f, "KeyLength::Bytes2048"),
            KeyLength::Bytes4096 => write!(f, "KeyLength::Bytes4096"),
        }
    }
}

impl std::error::Error for KeyLength {
    fn description(&self) -> &str {
        match self {
            KeyLength::Bytes8 => "The key is not 8 bytes in length",
            KeyLength::Bytes16 => "The key is not 16 bytes in length",
            KeyLength::Bytes24 => "The key is not 24 bytes in length",
            KeyLength::Bytes32 => "The key is not 32 bytes in length",
            KeyLength::Bytes64 => "The key is not 64 bytes in length",
            KeyLength::Bytes128 => "The key is not 128 bytes in length",
            KeyLength::Bytes256 => "The key is not 256 bytes in length",
            KeyLength::Bytes512 => "The key is not 512 bytes in length",
            KeyLength::Bytes1024 => "The key is not 1024 bytes in length",
            KeyLength::Bytes2048 => "The key is not 2048 bytes in length",
            KeyLength::Bytes4096 => "The key is not 4096 bytes in length",
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        match self {
            KeyLength::Bytes8 => Some(&KeyLength::Bytes8),
            KeyLength::Bytes16 => Some(&KeyLength::Bytes16),
            KeyLength::Bytes24 => Some(&KeyLength::Bytes24),
            KeyLength::Bytes32 => Some(&KeyLength::Bytes32),
            KeyLength::Bytes64 => Some(&KeyLength::Bytes64),
            KeyLength::Bytes128 => Some(&KeyLength::Bytes128),
            KeyLength::Bytes256 => Some(&KeyLength::Bytes256),
            KeyLength::Bytes512 => Some(&KeyLength::Bytes512),
            KeyLength::Bytes1024 => Some(&KeyLength::Bytes1024),
            KeyLength::Bytes2048 => Some(&KeyLength::Bytes2048),
            KeyLength::Bytes4096 => Some(&KeyLength::Bytes4096),
        }
    }
}

/// Hardware Resources of a physical computer
/// ### Examples
/// ```
/// # use custom_codes::HardwareResources;
/// let foo = HardwareResources::CpuMaxed;
/// assert_eq!(foo, HardwareResources::CpuMaxed);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum HardwareResources {
    /// CPU is under heavy load
    CpuUnderHeavyLoad,
    /// RAM is under heavy load
    RamUnderHeavyLoad,
    /// CPU is maxed out
    CpuMaxed,
    /// CPU is Overclocked
    CpuOverclocked,
    /// CPU is underclocked eg. to save power
    CpuUnderclocked,
    /// Cpu is idle
    CpuIdle,
    /// CPU heat is normal
    CpuTempOk,
    /// CPU heat is high
    CpuThrottling,
    /// Etherned Card Available for use
    EthernetDevUp,
    ///Ethernet is unavailavle for use
    EthernetDevDown,
    /// Ethernet card accessed by processed
    EthernetDevAccessed,
    /// Carrier chip available
    CarrierDevUp,
    /// Carrier chip unavailable
    CarrierDevDown,
    /// Carrier chip does not exist in hardware
    CarrierDevChipMissing,
    ///Drivers are missing for components listed
    DriverMissing,
    ///Camera or optical module available
    OpticDevUp,
    ///Camera or optical module unavailable
    OpticDevDown,
    ///Camera or optical module accessed
    OpticalDevAccessed,
    ///Camera or optical module access is denied
    OpticalDevAccessDenied,
    /// Camera or optical module powered off
    OpticalDevPoweredOff,
    /// Optical chip does not exist in hardware
    OticalDevChipMissing,
    /// NFC device is available
    NfcDevUp,
    /// NFC device is unavailable
    NfcDevDown,
    /// NFC device is active
    NfcDevActive,
    /// NFC device is paired to other device
    NfcDevPaired,
    /// NFC device is accessed
    NfcDevAccessed,
    /// NFC device access is denied
    NfcDevDenied,
    /// NFC device is closed
    NfcDevClosed,
    /// NFC device is powered of
    NfcDevPoweredOff,
    /// WiFi module avaliable
    WiFiDevUp,
    /// WiFi module unavaliable
    WiFiDevDown,
    /// WiFi module access is denied,
    WiFiDevAccessDenied,
    /// WiFi module powered off
    WiFiDevPoweredOff,
    /// WiFi has been accessed by process
    WiFiDevAccessed,
    /// WiFi is in hotspot mode
    WiFiDevHotSpotMode,
    /// Bluetooth is available
    BluetoothDevUp,
    /// Bluetooth is paired
    BluetoothDevPaired,
    /// Bluetooth device is trusted
    BluetoothDevDevTrusted,
    /// Bluetooth has been untrusted
    BluetoothDevDevUntrusted,
    /// Bluetooth access is denied
    BluetoothDevAccessDenied,
    /// Bluetooth device has been unpaired
    BluetoothDevUnpaired,
    /// Bluetooth is in share mode for data
    BluetoothDevShareMode,
    /// Bluetooth is in audio or data sink
    BluetoothDevSinkMode,
    /// Bluetooth is transmitting
    BluetoothDevTxMode,
    /// Bluetooth is Receiving
    BluetoothDevRxMode,
    /// Bluetooth is closed
    BluetoothDevClosed,
    /// Bluetooth is powered off
    BluetoothDevPowerOff,
    /// Bluetooth is unavailable
    BluetoothDevDown,
    /// USB controller is available
    UsbDevUp,
    /// USB controller is available
    UsbDevDown,
    /// USB controller is powered off
    UsbDevPoweredOff,
    /// USB device has been added
    UsbDevAdded,
    /// USB is transmitting
    UsbDevTxMode,
    /// USB is recieving
    UsbDevRxMode,
    /// USB device has been removed
    UsbDevRemoved,
    /// USB device access is denied
    UsbDevAccessDenied,
    /// GPS module available
    GpsDevUp,
    /// GPS module unavailable
    GpsDevDown,
    /// GPS is transmitting
    GpsDevTxMode,
    /// GPS module receiving
    GpsDevRxmode,
    /// GPS Device access denied
    GpsDevAccessDenied,
    /// The device is in airplane mode
    AirplaneMode,
    /// Hardware not specified
    Unspecified,
}

/// Networking Specific
/// ### Examples
/// ```
/// # use custom_codes::Networking;
/// let foo = Networking::NetworkAccessDenied;
/// assert_eq!(foo, Networking::NetworkAccessDenied);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Networking {
    /// Permission to access network is denied
    NetworkAccessDenied,
    /// Network is available but unable to reach destination host
    HostUnreachable,
    /// Connection was reset and has been terminated
    ConnectionReset,
    /// Connection to the address has been stopped
    ConnectionAborted,
    /// The network operation failed because process has not finished connecting
    NotConnected,
    /// Headers for a particular protocol corrupted
    HeadersCorrupted,
    /// Server access was reached but didnt respond
    ServerNotResponding,
    /// IP Address is already in use
    IpInUse,
    /// Port is in use but IP is available
    PortInUse,
    /// The IP address and Port are both in use
    AddrInUse,
    /// IP Address is available
    IpAvailable,
    /// Network Operation Failed because a pipe is broken
    BrokenPipe,
    /// Connection already exists
    NetConnExists,
    /// Operation parameters are Invalid
    InvalidNetInput,
    /// Operation parameters are fine but operation data is invalid
    InvalidNetData,
    /// The network operation timed out
    NetTimedOut,
    /// The Network Operation was interrupted and can be resumed
    Interrupted,
    /// Unexpected end of connection
    UnexpectedNetEof,
    /// Connection Driver is buggy
    NetDriverBuggy,
    /// Network Unspecified
    Unspecified,
}

/// Hardware security access
/// ### Examples
/// ```
/// # use custom_codes::SecHardware;
/// let foo = SecHardware::UsbKeyUp;
/// assert_eq!(foo, SecHardware::UsbKeyUp);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum SecHardware {
    /// USB Stick Hardware Key is available
    UsbKeyUp,
    /// USB Stick Hardware Key is unavailable
    UsbKeyDown,
    /// USB Stick Hardware Key is transmitting,
    UsbKeyDevTxMode,
    /// For receiving updates and is not avaiable for use
    UsbKeyDevRxMode,
    /// Device access denied
    UsbKeyDevAccessDenied,
    /// Fingerprint device is available
    FingerPrintDevUp,
    /// Fingerprint device is unavailable
    FingerPrintDevDown,
    /// Fingerprint device is powered of
    FingerPrintDevPoweredOff,
    /// Fingerprint device is transmitting
    FingerPrintDevTxMode,
    /// For receiving updates and is not avaiable for use
    FingerPrintDevRxMode,
    /// Fingerprint Device access denied
    FingerPrintDevAccessDenied,
    /// Iris scanner is available
    IrisDevUp,
    /// Iris scanner is unavailable
    IrisDevDown,
    /// Iris scanner is powered off
    IrisDevPoweredOff,
    /// Iris scanner is transmitting
    IrisDevTxMode,
    /// For receiving updates and is not avaiable for use
    IrisDevRxMode,
    /// Iris Scanner Device access denied
    IrisDevAccessDenied,
    /// Infra-red device is available
    IrFloodDevUp,
    /// Infra-red device is unavailable
    IrFloodDevDown,
    /// Infra-red device is powered off
    IrFloodDevPoweredOff,
    /// Infra-red device is transmitting
    IrDevTxMode,
    /// For receiving updates and is not avaiable for use
    IrDevRxMode,
    /// Infra-red Device access denied
    IrDevAccessDenied,
    /// Hardware Unspecified
    Unspecified,
}
