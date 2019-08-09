use serde_derive::{Serialize, Deserialize};

/// Uniform `Type` for the cause of an operation
type Reason = String;

/// Give the Outcome of an operation
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Outcome {
    ///Result of Operation completed successfully
    Success,
    /// Result of Operation produced an error
    Failure(Reason),
    /// The result of the operation was forwarded to another operation
    Forward,
}

/// Activity status of token or access
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
    RejectedRac,
}

/// Creates a custom `boolean` value with more features than a Rustlang boolean which has only `true` or `false`
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum CustomBool {
    /// Similar to boolean true
    True,
    /// Similar to boolean false
    False,
    /// Option does not exist
    NonExist,
    /// Option not initialized
    NonInit,
}

/// Custom Response Codes Using Enums For Efficient Comparison
#[derive(Debug,PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum DbOps {
    /// A `Write` is unsuccessful
    Inserted,
    /// A `Change` is successful
    Unchanged,
    /// A `Command` is skipped since the document does not exist
    Skipped,
    /// Document already exists
    AlreadyExists,
    /// Document has been updated
    Modified,
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
    /// Log compaction successfull
    CompactionTrue,
    /// Log compaction unsuccessfull
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
    EncounteredErrors(Reason),
    /// No Access to the DB is allowed for this user or process
    DbPermissionDenied,
    /// Read Access to database is Denied
    ReadDenied,
    /// Write Access to database is Denied
    WriteDenied,
    /// Access to a certain Table or Denied is denied
    DocReadDenied,
    /// Write to a document is denied
    DocWriteDenied,
    /// Access to a field is denied
    FieldReadDenied,
    /// Writing to a Field is denied
    FieldWriteDenied,
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
    NotLogged(Reason),
}

/// Command Operations for execution and 
#[derive(Debug,PartialEq, Eq, Clone, Serialize, Deserialize)]
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
}

/// File operations
#[derive(Debug,PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum FileOps {
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
    EncounteredErrors(Reason),
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
    /// Delete a file if it exists,
    DeleteTrue,
    
}

/// Compression of files, bytes or directories
#[derive(Debug,PartialEq, Eq, Clone, Serialize, Deserialize)]
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
}


/// Status of a subscription that happens over a timespec
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Subscription {
    /// Subscription is active
    Subscribed,
    /// Subscription is inactive
    Unsubscribed,
    /// Subscription is not yet activated
    NotActivated,
    /// Subscription does not exist
    NonExist,
    /// Subscription operation has not been initialized
    NonInit,
}

/// Status of an activity
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ActivityStatus {
    /// Status of an operation has been started
    Activated,
    /// Status of an operation has been stopped
    Deactivated,
    /// Status of an operation does not exist
    NonExist,
    /// Status of an operation is not initialized
    NonInit,
}

/// Initialize an activity
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum ActivityInit {
    /// Activate an operation
    Activate,
    /// Deactivate an operation
    Deactivate,
    /// Activity operation does not exists
    NonExist,
    /// Initialize an operation but dont execute
    NonInit,
}

/// Command Line Options
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Cli {
    /// Parsing
    ParseInProgress,
    /// Parsing Async
    ParseAsync,
    /// Not enough commandline arguments
    NotEnoughArgs,
    /// Encountered parsing errors
    EncounteredErrors(Reason),
}

/// Date and time custom codes
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
#[derive(Debug,PartialEq, Eq, Clone, Serialize, Deserialize)]
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
    KeyGenTrue,
    /// Key generation failed
    KeyGenFailed,
    /// Key has been deleted
    KeyDeleted,
    /// random data generated from Cryptographically Secure PRNG (CSPRNG)
    CryptoRandomGenerated,
    /// random data generated from Noncryptographic PRNG
    NonCryptoRandomGenerated,
    /// Message Authentication Code is authentic
    AuthenticMAC,
    /// Message Authentication Code is corrupted
    InvalidMAC,
    /// Random Authentication Code (RAC) Token is genuine/authentic
    ValidRAC,
    ///  Random Authentication Code (RAC) Token is not genuine/authentic
    InvalidRAC,
}

/// Hardware Resources of a physical computer
#[derive(Debug,PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum HardwareResources {
    /// CPU is under heavy load
    CpuUnderHeavyLoad,
    /// RAM is under heavy load
    RamUnderHeavyLoad,
    /// CPU is maxed out
    CpuMaxed,
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
}


/// Networking Specific
#[derive(Debug,PartialEq, Eq, Clone, Serialize, Deserialize)]
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
    /// The network connection was interrupted while in progress
    Interrupted,
    /// Unexpected end of connection
    UnexpectedNetEof,
    /// Connection Driver is buggy
    NetDriverBuggy,
    /// An error ccured because of a malfunction or bug
    Other(Reason),
}

/// Hardware security access
#[derive(Debug,PartialEq, Eq, Clone, Serialize, Deserialize)]
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

}
