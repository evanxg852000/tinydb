#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NotLeader {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
    #[prost(message, optional, tag = "2")]
    pub leader: ::core::option::Option<super::metapb::Peer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoreNotMatch {
    #[prost(uint64, tag = "1")]
    pub request_store_id: u64,
    #[prost(uint64, tag = "2")]
    pub actual_store_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegionNotFound {
    #[prost(uint64, tag = "1")]
    pub region_id: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyNotInRegion {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub region_id: u64,
    #[prost(bytes = "vec", tag = "3")]
    pub start_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub end_key: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EpochNotMatch {
    #[prost(message, repeated, tag = "1")]
    pub current_regions: ::prost::alloc::vec::Vec<super::metapb::Region>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StaleCommand {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(string, tag = "1")]
    pub message: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub not_leader: ::core::option::Option<NotLeader>,
    #[prost(message, optional, tag = "3")]
    pub region_not_found: ::core::option::Option<RegionNotFound>,
    #[prost(message, optional, tag = "4")]
    pub key_not_in_region: ::core::option::Option<KeyNotInRegion>,
    #[prost(message, optional, tag = "5")]
    pub epoch_not_match: ::core::option::Option<EpochNotMatch>,
    #[prost(message, optional, tag = "7")]
    pub stale_command: ::core::option::Option<StaleCommand>,
    #[prost(message, optional, tag = "8")]
    pub store_not_match: ::core::option::Option<StoreNotMatch>,
}
