use crate::kv::{config::Config, Storage};

use super::{disk::DiskStorage, memory::MemoryStorage};


#[derive(Debug)]
enum InnerStore {
    Memory(MemoryStorage),
    Disk(DiskStorage),
}

#[derive(Debug)]
pub struct StandaloneStorage {
    storage: InnerStore,
}

impl StandaloneStorage {
    
    pub fn new(config: Config) -> Self {
        Self {
            storage: InnerStore::Disk(DiskStorage::new(config.db_path).unwrap())
        }
    }

    pub fn in_memory() -> Self {
        Self {
            storage: InnerStore::Memory(MemoryStorage::new())
        }
    }

}

impl Storage for StandaloneStorage {
    fn start(&self) -> crate::kv::error::TkvResult<()> {
        match &self.storage {
            InnerStore::Memory(storage) => storage.start(),
            InnerStore::Disk(storage) => storage.start(),
        }
    }

    fn stop(self) -> crate::kv::error::TkvResult<()> {
        match self.storage {
            InnerStore::Memory(storage) => storage.stop(),
            InnerStore::Disk(storage) => storage.stop(),
        }
    }

    fn write(&self, batch: Vec<super::mutation::Mutation>) -> crate::kv::error::TkvResult<()> {
        match &self.storage {
            InnerStore::Memory(storage) => storage.write(batch),
            InnerStore::Disk(storage) => storage.write(batch),
        }
    }

    fn get(&self, cf: crate::kv::ColumnFamily, key: &[u8]) -> crate::kv::error::TkvResult<Option<Vec<u8>>> {
        match &self.storage {
            InnerStore::Memory(storage) => storage.get(cf, key),
            InnerStore::Disk(storage) => storage.get(cf, key),
        }
    }

    fn scan(&self, cf: crate::kv::ColumnFamily, start: std::ops::Bound<Vec<u8>>, end: std::ops::Bound<Vec<u8>>) -> crate::kv::error::TkvResult<Box<dyn crate::kv::StorageScanner + '_>> {
        match &self.storage {
            InnerStore::Memory(storage) => storage.scan(cf, start, end),
            InnerStore::Disk(storage) => storage.scan(cf, start, end),
        }
    }
}
