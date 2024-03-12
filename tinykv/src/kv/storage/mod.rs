use std::ops::Bound;

pub mod mutation;
pub mod memory;
pub mod disk;
pub mod raft;
pub mod standalone;


#[derive(Debug, Clone)]
struct ByteArrayRangeBound(Bound<Vec<u8>>, Bound<Vec<u8>>);

impl std::ops::RangeBounds<[u8]> for ByteArrayRangeBound {
    fn start_bound(&self) -> Bound<&[u8]> {
        match &self.0 {
            Bound::Included(v) => Bound::Included(v.as_slice()),
            Bound::Excluded(v) => Bound::Excluded(v.as_slice()),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    fn end_bound(&self) -> Bound<&[u8]> {
        match &self.1 {
            Bound::Included(v) => Bound::Included(v.as_slice()),
            Bound::Excluded(v) => Bound::Excluded(v.as_slice()),
            Bound::Unbounded => Bound::Unbounded,
        }
    }
    
}

