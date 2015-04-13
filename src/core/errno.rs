// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]

use std::fmt::{self, Debug, Formatter};
use std::convert::{From};
use std::io::{ErrorKind};

use cty::{self, c_int};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errno(pub c_int);

macro_rules! create {
    ($($name:ident = ($val:expr, $str:expr),)*) => {
        $(pub const $name: Errno = Errno($val);)*

        impl Errno {
            pub fn as_str(self) -> &'static str {
                match self {
                    $($name => $str,)*
                    _ => "Unknown error",
                }
            }
        }

        impl Debug for Errno {
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                match *self {
                    $($name => f.write_str(stringify!($name)),)*
                    x => write!(f, "Unknown({})", x.0),
                }
            }
        }
    }
}

create! {
    NotPermitted            = (cty::EPERM           , "Operation not permitted"),
    DoesNotExist            = (cty::ENOENT          , "No such file or directory"),
    NoSuchProcess           = (cty::ESRCH           , "No process matches the specified process ID"),
    Interrupted             = (cty::EINTR           , "Function call interrupted"),
    InputOutput             = (cty::EIO             , "Input/Output error"),
    NoSuchDevice            = (cty::ENXIO           , "No such device or address"),
    TooManyArguemnts        = (cty::E2BIG           , "Argument list too long"),
    InvalidExecutable       = (cty::ENOEXEC         , "Invalid executable file format"),
    BadFileDesc             = (cty::EBADF           , "Bad file descriptor"),
    NoChildProcesses        = (cty::ECHILD          , "There are no child processes"),
    WouldBlock              = (cty::EAGAIN          , "Resource temporarily unavailable"),
    NoMemory                = (cty::ENOMEM          , "No memory available"),
    AccessDenied            = (cty::EACCES          , "Permission denied"),
    InvalidPointer          = (cty::EFAULT          , "Invalid pointer"),
    NoBlockSpecialFile      = (cty::ENOTBLK         , "Resource busy"),
    ResourceBusy            = (cty::EBUSY           , "Block special file required"),
    FileExists              = (cty::EEXIST          , "File exists"),
    CrossFileSystemLink     = (cty::EXDEV           , "Attempted to link across file systems"),
    WrongDeviceType         = (cty::ENODEV          , "Wrong device type for operation"),
    NotADirectory           = (cty::ENOTDIR         , "Directory required for operation"),
    IsADirectory            = (cty::EISDIR          , "Directory not permitted in operation"),
    InvalidArgument         = (cty::EINVAL          , "Invalid argument"),
    SystemFileLimit         = (cty::ENFILE          , "Process file limit reached"),
    ProcessFileLimit        = (cty::EMFILE          , "System file limit reached"),
    NotATerminal            = (cty::ENOTTY          , "Argument is not a terminal"),
    ExecutableBusy          = (cty::ETXTBSY         , "Trying to execute and write a file at the same time"),
    FileTooBig              = (cty::EFBIG           , "File too big"),
    DeviceFull              = (cty::ENOSPC          , "No space left on device"),
    InvalidSeek             = (cty::ESPIPE          , "Invalid seek operation"),
    ReadOnlyFileSystem      = (cty::EROFS           , "Operation not permitted on read-only file system"),
    TooManyLinks            = (cty::EMLINK          , "Too many links"),
    BrokenPipe              = (cty::EPIPE           , "Broken pipe"),
    DomainError             = (cty::EDOM            , "Domain error"),
    RangeError              = (cty::ERANGE          , "Range error"),
    DeadlockAvoided         = (cty::EDEADLK         , "Deadlock avoided"),
    PathTooLong             = (cty::ENAMETOOLONG    , "Path too long"),
    NoLocksAvailable        = (cty::ENOLCK          , "No locks available"),
    NotImplemented          = (cty::ENOSYS          , "Function not implemented"),
    NotEmpty                = (cty::ENOTEMPTY       , "Directory not empty"),
    TooManySymlinks         = (cty::ELOOP           , "Too many levels of symbolic links"),
    NoMessageOfType         = (cty::ENOMSG          , "No message of desired type"),
    IdentifierRemoved       = (cty::EIDRM           , "Identifier removed"),
    ChannelOutOfRange       = (cty::ECHRNG          , "Channel number out of range"),
    Level2NotSync           = (cty::EL2NSYNC        , "Level 2 not synchronized"),
    Level3Halted            = (cty::EL3HLT          , "Level 3 halted"),
    Level3Reset             = (cty::EL3RST          , "Level 3 reset"),
    LinkNumberOutOfRange    = (cty::ELNRNG          , "Link number out of range"),
    ProtoDriverNotAttached  = (cty::EUNATCH         , "Protocol driver not attached"),
    NoCSIStructAvailable    = (cty::ENOCSI          , "No CSI structure available"),
    Level2Halted            = (cty::EL2HLT          , "Level 2 halted"),
    InvalidExchange         = (cty::EBADE           , "Invalid exchange"),
    InvalidReqDesc          = (cty::EBADR           , "Invalid request descriptor"),
    ExchangeFull            = (cty::EXFULL          , "Exchange full"),
    NoAnode                 = (cty::ENOANO          , "No anode"),
    InvalidRequestCode      = (cty::EBADRQC         , "Invalid request code"),
    InvalidSlot             = (cty::EBADSLT         , "Invalid slot"),
    BadFontFileFormat       = (cty::EBFONT          , "Bad font file format"),
    NotAStream              = (cty::ENOSTR          , "Device not a stream"),
    NoDataAvailable         = (cty::ENODATA         , "No data available"),
    TimerExpired            = (cty::ETIME           , "Timer expired"),
    OutOfStreamsResources   = (cty::ENOSR           , "Out of streams resources"),
    NotOnNetwork            = (cty::ENONET          , "Machine is not on the network"),
    PackageNotInstalled     = (cty::ENOPKG          , "Package not installed"),
    ObjectIsRemote          = (cty::EREMOTE         , "Object is remote"),
    LinkSevered             = (cty::ENOLINK         , "Link has been severed"),
    AdvertiseError          = (cty::EADV            , "Advertise error"),
    SrmountError            = (cty::ESRMNT          , "Srmount error"),
    CommunitacionError      = (cty::ECOMM           , "Communication error on send"),
    ProtocolError           = (cty::EPROTO          , "Protocol error"),
    MultihopAttempted       = (cty::EMULTIHOP       , "Multihop attempted"),
    RFSError                = (cty::EDOTDOT         , "RFS specific error"),
    NotADataMessage         = (cty::EBADMSG         , "Not a data message"),
    Overflow                = (cty::EOVERFLOW       , "Value too large for defined data type"),
    NotUnique               = (cty::ENOTUNIQ        , "Name not unique on network"),
    BadFileDescState        = (cty::EBADFD          , "File descriptor in bad state"),
    RemoteAddrChanged       = (cty::EREMCHG         , "Remote address changed"),
    SharedLibInaccessible   = (cty::ELIBACC         , "Can not access a needed shared library"),
    SharedLibCorrupted      = (cty::ELIBBAD         , "Accessing a corrupted shared library"),
    LibSectionCorrupted     = (cty::ELIBSCN         , ".lib section in a.out corrupted"),
    TooManySharedLibs       = (cty::ELIBMAX         , "Attempting to link in too many shared libraries"),
    SharedLibExec           = (cty::ELIBEXEC        , "Cannot exec a shared library directly"),
    InvalidSequence         = (cty::EILSEQ          , "Invalid sequence"),
    Restart                 = (cty::ERESTART        , "Interrupted system call should be restarted"),
    StreamPipeError         = (cty::ESTRPIPE        , "Streams pipe error"),
    TooManyUsers            = (cty::EUSERS          , "Too many users"),
    NotASocket              = (cty::ENOTSOCK        , "Argument is not a socket"),
    NoDefaultDestination    = (cty::EDESTADDRREQ    , "Connectionless socket has no destination"),
    MessageSize             = (cty::EMSGSIZE        , "Message too large"),
    ProtoNotSupported       = (cty::EPROTOTYPE      , "Protocol not supported by socket type"),
    OpNotSupported          = (cty::ENOPROTOOPT     , "Operation not supported by protocol"),
    ProtoNotSupported2      = (cty::EPROTONOSUPPORT , "Protocol not supported by socket domain"),
    SocketTypeNotSupported  = (cty::ESOCKTNOSUPPORT , "Socket type is not supported"),
    NotSupported            = (cty::EOPNOTSUPP      , "Operation not supported"),
    ProtoFamilyNotSupported = (cty::EPFNOSUPPORT    , "Protocol family not supported"),
    AddrFamilyNotSupported  = (cty::EAFNOSUPPORT    , "Address family not supported"),
    AddressInUse            = (cty::EADDRINUSE      , "Socket address already in use"),
    AddressNotAvailable     = (cty::EADDRNOTAVAIL   , "Socket address is not available"),
    NetworkDown             = (cty::ENETDOWN        , "Network is down"),
    NetworkUnreachable      = (cty::ENETUNREACH     , "Remote network is unreachable"),
    HostCrashed             = (cty::ENETRESET       , "Remote hast crashed"),
    ConnectionAborted       = (cty::ECONNABORTED    , "Connection locally aborted"),
    ConnectionReset         = (cty::ECONNRESET      , "Connection closed"),
    KernelBuffersBusy       = (cty::ENOBUFS         , "All kernel I/O buffers are in use"),
    SocketConnected         = (cty::EISCONN         , "Socket is already connected"),
    SocketNotConnected      = (cty::ENOTCONN        , "Socket is not connected"),
    SocketShutDown          = (cty::ESHUTDOWN       , "Socket has shut down"),
    TooManyReferences       = (cty::ETOOMANYREFS    , "Too many references"),
    SocketTimedOut          = (cty::ETIMEDOUT       , "Socket operation timed out"),
    ConnectionRefused       = (cty::ECONNREFUSED    , "Remote host is down"),
    HostDown                = (cty::EHOSTDOWN       , "Remote host is unreachable"),
    HostUnreachable         = (cty::EHOSTUNREACH    , "Remote host refused connection"),
    AlreadyInProgress       = (cty::EALREADY        , "Operation already in progress"),
    OperationInitiated      = (cty::EINPROGRESS     , "Operation initiated"),
    StaleFileHandle         = (cty::ESTALE          , "Stale file handle"),
    NeedsCleaning           = (cty::EUCLEAN         , "Structure needs cleaning"),
    NotXENIX                = (cty::ENOTNAM         , "Not a XENIX named type file"),
    NoXENIXSemaphores       = (cty::ENAVAIL         , "No XENIX semaphores available"),
    NamedTypeFile           = (cty::EISNAM          , "Is a named type file"),
    RemoteIOError           = (cty::EREMOTEIO       , "Remote I/O error"),
    DiskQuota               = (cty::EDQUOT          , "Disk quota exceeded"),
    NoMedium                = (cty::ENOMEDIUM       , "No medium found"),
    WrongMediumType         = (cty::EMEDIUMTYPE     , "Wrong medium type"),
    OperationCanceled       = (cty::ECANCELED       , "Asynchronous operation canceled"),
    KeyNotAvailable         = (cty::ENOKEY          , "Required key not available"),
    KeyExpired              = (cty::EKEYEXPIRED     , "Key has expired"),
    KeyRevoked              = (cty::EKEYREVOKED     , "Key has been revoked"),
    KeyRejected             = (cty::EKEYREJECTED    , "Key was rejected by service"),
    OwnerDied               = (cty::EOWNERDEAD      , "Owner died"),
    IrrecoverableState      = (cty::ENOTRECOVERABLE , "State not recoverable"),
    RFKill                  = (cty::ERFKILL         , "Operation not possible due to RF-kill"),
    HardwarePoison          = (cty::EHWPOISON       , "Memory page has hardware error"),

    RustError               = (5000, "Rust custom error"),
}

impl From<Errno> for ::std::io::Error {
    fn from(e: Errno) -> ::std::io::Error {
        ::std::io::Error::from_raw_os_error(e.0)
    }
}

impl From<::std::io::Error> for Errno {
    fn from(e: ::std::io::Error) -> Errno {
        if let Some(num) = e.raw_os_error() {
            return Errno(num as c_int);
        }
        match e.kind() {
            ErrorKind::PermissionDenied               => NotPermitted,
            ErrorKind::NotFound                       => DoesNotExist,
            ErrorKind::ConnectionRefused              => ConnectionRefused,
            ErrorKind::ConnectionReset                => ConnectionReset,
            ErrorKind::ConnectionAborted              => ConnectionAborted,
            ErrorKind::NotConnected                   => SocketNotConnected,
            ErrorKind::AddrInUse                      => AddressInUse,
            ErrorKind::AddrNotAvailable               => AddressNotAvailable,
            ErrorKind::BrokenPipe                     => BrokenPipe,
            ErrorKind::AlreadyExists                  => FileExists,
            ErrorKind::WouldBlock                     => WouldBlock,
            ErrorKind::InvalidInput                   => InvalidArgument,
            ErrorKind::TimedOut                       => SocketTimedOut,
            ErrorKind::WriteZero                      => RustError,
            ErrorKind::Interrupted                    => Interrupted,
            ErrorKind::Other                          => RustError,
            _                                         => RustError,
        }
    }
}
