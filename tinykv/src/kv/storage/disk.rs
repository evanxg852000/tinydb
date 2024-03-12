
use std::fmt::Debug;
use std::ops::Bound;
use std::path::Path;

use heed::{Database, Env, EnvOpenOptions, RoRange, RoTxn};
use heed::types::*;
use ouroboros::self_referencing;


use crate::kv::error::{TkvError, TkvResult};
use crate::kv::{ColumnFamily, KvIterator, Storage, StorageScanner};

use super::mutation::Mutation;
use super::ByteArrayRangeBound;

pub struct DiskStorage {
    env: Env,
    store: Database<CowSlice<u8>, CowSlice<u8>>,
}

impl Debug for DiskStorage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DiskStorage")
            .field("env", &self.env.path())
            .finish()
    }
}

impl DiskStorage {

    pub fn new<P: AsRef<Path>>(path: P) -> TkvResult<Self> {
        let env = EnvOpenOptions::new()
            .map_size(1 * 1024 * 1024 * 1024) // 1GiB
            .open(&path)?;
        if !path.as_ref().exists() {
            std::fs::create_dir_all(path)?;
        }

        let db: Database<CowSlice<u8>, CowSlice<u8>> =  env.open_database(None)?
            .unwrap_or_else(|| env.create_database(None).expect("unable to create db"));

    
        Ok(Self {env, store: db})
    }

}

impl Storage for DiskStorage {
    fn start(&self) -> TkvResult<()> {
        Ok(())
    }

    fn stop(self) -> TkvResult<()> {
        let closing_event = self.env.prepare_for_closing();
        closing_event.wait();
        Ok(())
    }

    fn write(&self, batch: Vec<Mutation>) -> TkvResult<()> {
        let mut write_txn = self.env.write_txn()?;
        for mutation in batch {
            match mutation {
                Mutation::Put { key, value, cf } => {
                    self.store.put(&mut write_txn, &cf.add_prefix(&key), &value)?;
                },
                Mutation::Delete { key, cf } => {
                    self.store.delete(&mut write_txn, &cf.add_prefix(&key))?;
                },
            };
        }
        write_txn.commit()?;
        Ok(())
    }

    fn get(&self, cf: ColumnFamily, key: &[u8]) -> TkvResult<Option<Vec<u8>>> {
        let read_txn = self.env.read_txn()?;
        self.store
            .get(&read_txn, &cf.add_prefix(key))
            .map(|opt| opt.map(|v| v.to_vec()))
            .map_err(TkvError::from)
    }

    fn scan(&self, cf: ColumnFamily, start: Bound<Vec<u8>>, end: Bound<Vec<u8>>) -> TkvResult<Box<dyn StorageScanner + '_>> {
        Ok(Box::new(DiskScanner::new(self, cf, start, end)))
    }

}


// DiskScanner
pub struct DiskScanner<'a > {
    cf: ColumnFamily,
    storage: &'a DiskStorage,  // The underlying storage.
    bound: ByteArrayRangeBound,
}

impl<'a> DiskScanner<'a > {
    pub fn new(
        storage: &'a DiskStorage,
        cf: ColumnFamily,
        start: Bound<Vec<u8>>,
        end: Bound<Vec<u8>>,
    ) -> Self {
        Self {
            cf,
            storage,
            bound: ByteArrayRangeBound(cf.add_bound_prefix(start), cf.add_bound_prefix(end)),
        }
    }
}

impl<'a> StorageScanner<'a> for DiskScanner<'a> {
    
    fn iter(&self) -> Box<dyn KvIterator + '_> {
        let disk_iter = DiskStorageIteratorBuilder {
            cf: self.cf,
            read_txn: self.storage.env.read_txn().unwrap(),
            inner_builder: |read_txn| self.storage.store.range(&read_txn, &self.bound).unwrap(),
        }
        .build();
        Box::new(disk_iter)
    }

}


#[self_referencing]
struct DiskStorageIterator<'a> {
    cf: ColumnFamily,
    read_txn: RoTxn<'a>,
    #[borrows(read_txn)]
    #[not_covariant]
    inner: RoRange<'this, CowSlice<u8>, CowSlice<u8>>,
}

impl<'a> Iterator for DiskStorageIterator<'a> {
    type Item = TkvResult<(Vec<u8>, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        let cf = self.borrow_cf().clone();
        self
            .with_inner_mut(|inner| inner.next())
            .map(|result| 
                result
                    .map(|(key, value)| (cf.strip_prefix(&key), value.to_vec()))
                    .map_err(TkvError::from)
            )
    }
}


#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;
    super::super::super::tests::test_storage!({
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("data");
        std::fs::create_dir_all(&db_path).unwrap();
        (DiskStorage::new(db_path.to_string_lossy().to_string()).unwrap(), ColumnFamily::Default)
    });
}
