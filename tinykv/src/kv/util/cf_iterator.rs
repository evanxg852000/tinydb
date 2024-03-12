use bytes::Bytes;

use crate::kv::ColumnFamily;


#[derive(Debug)]
pub struct DbItem {
    pub cf: ColumnFamily,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

pub trait DBIterator {
    // Returns pointer to the current key/value pair.
    fn item(&self) -> DbItem;

    // Returns false when iterator is exhausted.
    fn is_valid(&self) -> bool;

    // Advances the iterator by one. Always check if iter.is_valid() 
    // after a calling next() to ensure you have access to a valid DbItem.
	fn next(&self);

    // Seek would seek to the provided key if present. If absent, it would seek to the next smallest key
	// greater than provided.
	fn seek(&self, key: Bytes);

	// Close the iterator
	fn close(&self); 
}



