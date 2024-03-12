
pub mod errorpb {
    include!("errorpb.rs");
}

pub mod metapb {
    include!("metapb.rs");
}

pub mod kvpb {
    include!("kvpb.rs");
}

pub mod tinykv {
    include!("tinykvpb.rs");
}

pub const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("reflection-descriptor.bin");


