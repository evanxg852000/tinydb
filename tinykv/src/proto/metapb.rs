#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// max peer count for a region.
    /// scheduler will do the auto-balance if region peer count mismatches.
    ///
    /// more ...
    #[prost(uint32, tag = "2")]
    pub max_peer_count: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Store {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// address to handle client requests (kv, cop, etc.)
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "StoreState", tag = "3")]
    pub state: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionEpoch {
    /// config change version, auto-incremented when peer is added or removed.
    #[prost(uint64, tag = "1")]
    pub config_version: u64,
    /// region version, auto-increnented when region is split or merged.
    #[prost(uint64, tag = "2")]
    pub version: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Region {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Region key range [start_key, end_key).
    #[prost(bytes = "vec", tag = "2")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub region_epoch: ::core::option::Option<RegionEpoch>,
    #[prost(message, repeated, tag = "5")]
    pub peers: ::prost::alloc::vec::Vec<Peer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Peer {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub store_id: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoreState {
    Up = 0,
    Offline = 1,
    Tomstone = 2,
}
impl StoreState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StoreState::Up => "Up",
            StoreState::Offline => "Offline",
            StoreState::Tomstone => "Tomstone",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Up" => Some(Self::Up),
            "Offline" => Some(Self::Offline),
            "Tomstone" => Some(Self::Tomstone),
            _ => None,
        }
    }
}
