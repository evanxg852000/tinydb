pub mod config;
pub mod storage;
pub mod util;
pub mod error;
pub mod server;


use std::ops::Bound;

use strum::EnumString;

use self::{error::TkvResult, storage::mutation::Mutation};


pub trait KvIterator: Iterator<Item = TkvResult<(Vec<u8>, Vec<u8>)>> {}

impl<I: Iterator<Item = TkvResult<(Vec<u8>, Vec<u8>)>>> KvIterator for I{}

#[derive(Debug, Copy, Clone, PartialEq, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ColumnFamily {
    Default,
    Write,
    Lock,
}

impl ColumnFamily {
    pub fn add_prefix(&self, key: &[u8]) -> Vec<u8> {
        let mut real_key = match self {
            ColumnFamily::Default => b"default_".to_vec(),
            ColumnFamily::Write => b"write_".to_vec(),
            ColumnFamily::Lock => b"lock_".to_vec(),
        };
        real_key.extend(key);
        real_key
    }

    pub fn add_bound_prefix(&self, bound: Bound<Vec<u8>>) -> Bound<Vec<u8>> {
        match  bound {
            Bound::Included(v) => Bound::Included(self.add_prefix(&v)),
            Bound::Excluded(v) => Bound::Excluded(self.add_prefix(&v)),
            Bound::Unbounded => Bound::Unbounded,
        }
    }

    pub fn strip_prefix(&self, key: &[u8]) -> Vec<u8> {
        let real_key = match self {
            ColumnFamily::Default => key.strip_prefix(b"default_"),
            ColumnFamily::Write => key.strip_prefix(b"write_"),
            ColumnFamily::Lock => key.strip_prefix(b"lock_"),
        };
        real_key.unwrap().to_vec()
    }
}


/// Storage represents the internal-facing server part of TinyYKV,
/// It handles sending and receiving messages from other TinyKV nodes.
/// As part of that responsibility, it also reads and writes data to disk
/// (or semi-permanent memory).
pub trait Storage: std::fmt::Debug + Send + Sync {
    fn start(&self) -> TkvResult<()>;
    
    fn stop(self) -> TkvResult<()>;
    
    fn write(&self, batch: Vec<Mutation>) -> TkvResult<()>;
    
    fn get(&self, cf: ColumnFamily, key: &[u8]) -> TkvResult<Option<Vec<u8>>>;
    
    fn put(&self, cf: ColumnFamily, key: &[u8], value: Vec<u8>) -> TkvResult<()> {
        let mutation = Mutation::Put { key: key.to_vec(), value, cf };
        self.write(vec![mutation])
    }

    fn delete(&self, cf: ColumnFamily, key: &[u8]) -> TkvResult<()> {
        let mutation = Mutation::Delete { key: key.to_vec(), cf };
        self.write(vec![mutation])
    }

    fn scan(&self, cf: ColumnFamily, start: Bound<Vec<u8>>, end: Bound<Vec<u8>>) -> TkvResult<Box<dyn StorageScanner + '_>>;
}

pub trait StorageScanner<'a> {
    fn iter(&self) -> Box<dyn KvIterator + '_>;
}


#[cfg(test)]
mod tests {
    // Generates common tests for any Storage implementation
    macro_rules! test_storage {
        ($setup:expr) => {

            // assert an iterator
            #[track_caller]
            fn assert_iter<I>(iter: I, expect: Vec<(Vec<u8>, Vec<u8>)>) -> TkvResult<()>
            where
                I: Iterator<Item = TkvResult<(Vec<u8>, Vec<u8>)>>,
            {
                assert_eq!(
                    iter.collect::<TkvResult<Vec<_>>>()?,
                    expect,
                );
                Ok(())
            }

            // test storage point operations, i.e set, get, delete.
            #[test]
            fn point_operations() -> TkvResult<()> {
                let (storage, cf) = $setup;

                // getting a missing key should return None.
                assert_eq!(storage.get(cf, b"a")?, None);

                // setting & getting a key should return its value
                storage.put(cf, b"a", vec![1])?;
                assert_eq!(storage.get(cf, b"a")?, Some(vec![1]));

                // setting a different key should not affect the first.
                storage.put(cf, b"b", vec![2])?;
                assert_eq!(storage.get(cf, b"b")?, Some(vec![2]));
                assert_eq!(storage.get(cf, b"a")?, Some(vec![1]));

                // getting a different non-existing key should return None.
                // strong comparison is case sensitive.
                assert_eq!(storage.get(cf, b"c")?, None);
                assert_eq!(storage.get(cf, b"A")?, None);

                // setting an existing key should replace its value.
                storage.put(cf, b"a", vec![0])?;
                assert_eq!(storage.get(cf, b"a")?, Some(vec![0]));

                // deleting a key should remove it, but not affect others.
                storage.delete(cf, b"a")?;
                assert_eq!(storage.get(cf, b"a")?, None);
                assert_eq!(storage.get(cf, b"b")?, Some(vec![2]));

                // deletes are idempotent.
                storage.delete(cf, b"a")?;
                assert_eq!(storage.get(cf, b"a")?, None);

                Ok(())
            }

            // tests storage on empty key and values
            #[test]
            fn point_operations_with_empty() -> TkvResult<()> {
                let (storage, cf) = $setup;

                assert_eq!(storage.get(cf, b"")?, None);
                storage.put(cf, b"", vec![])?;
                assert_eq!(storage.get(cf, b"")?, Some(vec![]));
                storage.delete(cf, b"")?;
                assert_eq!(storage.get(cf, b"")?, None);
                Ok(())
            }

            // tests storage on values of increasing sizes, up to 16 MB.
            #[test]
            fn point_operations_sizes() -> TkvResult<()> {
                let (storage, cf) = $setup;
                // generate values in increasing powers of two.
                for it in (1..=24) {
                    let key = format!("x_{}", it).as_bytes().to_vec();
                    let value = "x".repeat(1 << it).as_bytes().to_vec();

                    assert_eq!(storage.get(cf, &key)?, None);
                    storage.put(cf, &key, value.clone())?;
                    assert_eq!(storage.get(cf, &key)?, Some(value));
                    storage.delete(cf, &key)?;
                    assert_eq!(storage.get(cf, &key)?, None);
                }
                Ok(())
            }

            #[test]
            fn scan() -> TkvResult<()> {
                let (storage, cf) = $setup;

                storage.put(cf, b"a", vec![1])?;
                storage.put(cf, b"b", vec![2])?;
                storage.put(cf, b"ba", vec![2, 1])?;
                storage.put(cf, b"bb", vec![2, 2])?;
                storage.put(cf, b"c", vec![3])?;
                storage.put(cf, b"C", vec![3])?;

                // forward scans.
                {
                    let scanner = storage.scan(cf, Bound::Included(b"b".to_vec()), Bound::Excluded(b"bz".to_vec()))?;
                    assert_iter(
                        scanner.iter(),
                        vec![(b"b".to_vec(), vec![2]), (b"ba".to_vec(), vec![2, 1]), (b"bb".to_vec(), vec![2, 2])],
                    )?;
                }

                //TODO: add more cases (half open) 

                // full range.
                {
                    let scanner =  storage.scan(cf, Bound::Unbounded, Bound::Unbounded)?;
                    assert_iter(
                        scanner.iter(),
                        vec![
                            (b"C".to_vec(), vec![3]),
                            (b"a".to_vec(), vec![1]),
                            (b"b".to_vec(), vec![2]),
                            (b"ba".to_vec(), vec![2, 1]),
                            (b"bb".to_vec(), vec![2, 2]),
                            (b"c".to_vec(), vec![3]),
                        ],
                    )?;
                }
                

                Ok(())
            }
            

        };
    }

    pub(super) use test_storage; // export for use in submodules
}
