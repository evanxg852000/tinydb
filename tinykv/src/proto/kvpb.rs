/// Raw commands.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGetRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawGetResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// True if the requested key doesn't exist; another error will not be signalled.
    #[prost(bool, tag = "4")]
    pub not_found: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawPutRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "4")]
    pub cf: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawPutResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDeleteRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "3")]
    pub cf: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawDeleteResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawScanRequest {
    #[prost(message, optional, tag = "1")]
    pub context: ::core::option::Option<Context>,
    #[prost(bytes = "vec", tag = "2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    /// The maximum number of values read.
    #[prost(uint32, tag = "3")]
    pub limit: u32,
    #[prost(string, tag = "4")]
    pub cf: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawScanResponse {
    #[prost(message, optional, tag = "1")]
    pub region_error: ::core::option::Option<super::errorpb::Error>,
    /// An error which affects the whole scan. Per-key errors are included in kvs.
    #[prost(string, tag = "2")]
    pub error: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub kvs: ::prost::alloc::vec::Vec<KvPair>,
}
/// Either a key/value pair or an error for a particular key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KvPair {
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<KeyError>,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mutation {
    #[prost(enumeration = "Op", tag = "1")]
    pub op: i32,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// Many responses can include a KeyError for some problem with one of the requested key.
/// Only one field is set and it indicates what the client should do in response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyError {
    /// Client should backoff or cleanup the lock then retry.
    #[prost(message, optional, tag = "1")]
    pub locked: ::core::option::Option<LockInfo>,
    /// Client may restart the txn. e.g write conflict.
    #[prost(string, tag = "2")]
    pub retryable: ::prost::alloc::string::String,
    /// Client should abort the txn.
    #[prost(string, tag = "3")]
    pub abort: ::prost::alloc::string::String,
    /// Another transaction is trying to write a key. The client can retry.
    #[prost(message, optional, tag = "4")]
    pub conflict: ::core::option::Option<WriteConflict>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LockInfo {
    #[prost(bytes = "vec", tag = "1")]
    pub primary_lock: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub lock_version: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "4")]
    pub lock_ttl: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteConflict {
    #[prost(uint64, tag = "1")]
    pub start_ts: u64,
    #[prost(uint64, tag = "2")]
    pub conflict_ts: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub primary: ::prost::alloc::vec::Vec<u8>,
}
/// Miscellaneous data present in each request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(message, optional, tag = "2")]
    pub region_epoch: ::core::option::Option<super::metapb::RegionEpoch>,
    #[prost(message, optional, tag = "3")]
    pub peer: ::core::option::Option<super::metapb::Peer>,
    #[prost(uint64, tag = "5")]
    pub term: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Op {
    Put = 0,
    Del = 1,
    Rollback = 2,
    /// Used by TinySQL but not TinyKV.
    Lock = 3,
}
impl Op {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Op::Put => "Put",
            Op::Del => "Del",
            Op::Rollback => "Rollback",
            Op::Lock => "Lock",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Put" => Some(Self::Put),
            "Del" => Some(Self::Del),
            "Rollback" => Some(Self::Rollback),
            "Lock" => Some(Self::Lock),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    NoAction = 0,
    /// The lock is rolled back because it has expired.
    TtlExpireRollback = 1,
    /// The lock does not exist, TinyKV left a record of the rollback, but did not
    /// have to delete a lock.
    LockNotExistRollback = 2,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::NoAction => "NoAction",
            Action::TtlExpireRollback => "TTLExpireRollback",
            Action::LockNotExistRollback => "LockNotExistRollback",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NoAction" => Some(Self::NoAction),
            "TTLExpireRollback" => Some(Self::TtlExpireRollback),
            "LockNotExistRollback" => Some(Self::LockNotExistRollback),
            _ => None,
        }
    }
}
