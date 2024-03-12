use std::{collections::BTreeMap, ops::Bound, sync::Arc};

use parking_lot::{Mutex, MutexGuard};

use crate::kv::{  error::TkvResult, ColumnFamily, KvIterator, Storage, StorageScanner};

use super::mutation::Mutation;


type BTree = BTreeMap<Vec<u8>, Vec<u8>>; 


#[derive(Debug)]
pub struct MemoryStorage {
    store: Arc<Mutex<BTree>>,
}

impl MemoryStorage {

    pub fn new() -> Self {
        Self {
            store: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }


}

impl Storage for MemoryStorage {
    fn start(&self) -> TkvResult<()> {
        Ok(())
    }

    fn stop(self) -> TkvResult<()> {
        Ok(())
    }

    fn write(&self, batch: Vec<Mutation>) -> TkvResult<()> {
        let mut lock_guard = self.store.lock();
        for mutation in batch {
            match mutation {
                Mutation::Put { key, value, cf } => lock_guard.insert(cf.add_prefix(&key), value),
                Mutation::Delete { key, cf } => lock_guard.remove(&cf.add_prefix(&key)),
            };
        }
        Ok(())
    }

    fn get(&self, cf: ColumnFamily, key: &[u8]) -> TkvResult<Option<Vec<u8>>> {
        let value = self.store.lock().get(&cf.add_prefix(key)).cloned();
        Ok(value)
    }

    fn scan(&self, cf: ColumnFamily, start: Bound<Vec<u8>>, end: Bound<Vec<u8>>) -> TkvResult<Box<dyn StorageScanner + '_>> {
        let storage = self.store.lock();
        Ok(Box::new(MemoryScanner::new(storage, cf, start, end)))
    }

}



// MemoryReader
pub struct MemoryScanner<'a > {
    storage: MutexGuard<'a, BTree>,  // The underlying storage.
    cf: ColumnFamily,
    bound: (Bound<Vec<u8>>, Bound<Vec<u8>>),
}

impl<'a> MemoryScanner<'a > {
    pub fn new(
        storage: MutexGuard<'a, BTree>,
        cf: ColumnFamily,
        start: Bound<Vec<u8>>,
        end: Bound<Vec<u8>>,
    ) -> Self {
        Self { 
            storage,
            cf: cf,
            bound: (cf.add_bound_prefix(start), cf.add_bound_prefix(end)),
        }
    }
}

impl<'a> StorageScanner<'a> for MemoryScanner<'a> {
    
    fn iter(&self) -> Box<dyn KvIterator + '_> {
        Box::new(MemoryStorageIterator {
            cf: self.cf,
            inner: self.storage.range(self.bound.clone())
        })
    }

}

struct MemoryStorageIterator<'a> {
    cf: ColumnFamily,
    inner: std::collections::btree_map::Range<'a, Vec<u8>, Vec<u8>>,
}

impl<'a> Iterator for MemoryStorageIterator<'a> {
    type Item = TkvResult<(Vec<u8>, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
            .map(|(key, value)| Ok((self.cf.strip_prefix(key), value.clone())))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    super::super::super::tests::test_storage!((MemoryStorage::new(), ColumnFamily::Default));
}
