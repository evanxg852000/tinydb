use std::{fs::{File, OpenOptions}, io::{ErrorKind, Read, Seek, SeekFrom, Write}, path::Path};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use serde::{ de::DeserializeOwned, Serialize};

use super::error::{RaftError, RaftResult};

pub struct RaftLog {
    offsets: Vec<usize>,
    next_item_offset: usize,
    file: File,
}

impl RaftLog {

    pub fn open<P: AsRef<Path>>(path: P) -> RaftResult<Self> {
        if !path.as_ref().exists() {
            File::create(path.as_ref())?;
        }
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(path)?;
        let (offsets, next_item_offset )=  Self::load_offsets(&mut file)?;
        Ok(Self { 
            offsets,
            next_item_offset,
            file,
        })
    }

    pub fn append<T: Serialize>(&mut self, item: T) -> RaftResult<()> {
        let data = bincode::serialize(&item)
            .map_err(RaftError::from)?;

        let length = data.len() as u64;

        self.offsets.push(self.next_item_offset);
        self.file.seek(SeekFrom::End(0))?;
        self.file.write_u64::<LittleEndian>(length)?;
        self.file.write_all(&data)?;
        self.file.flush()?;
        self.next_item_offset += 8 + data.len();

        Ok(())
    }

    pub fn get<'de, T: DeserializeOwned>(&mut self, index: usize) -> RaftResult<T> {
        let offset = self.offsets[index];
        self.file.seek(SeekFrom::Start(offset as u64))?;
     
        let length = self.file.read_u64::<LittleEndian>()?;
        let mut data = vec![0u8; length as usize];
        self.file.read_exact(&mut data)?;
        bincode::deserialize::<T>(&data)
            .map_err(RaftError::from)
    }

    // get rid of unnecessary log entries from 0 (start of file) 
    // up to index (included).
    pub fn truncate(&mut self, index: usize) -> RaftResult<()> {
        let next_valid_offset = self.offsets[index + 1];
        self.file.seek(SeekFrom::Start(next_valid_offset as u64))?;

        let mut data = vec![];
        self.file.read_to_end(&mut data)?;
        assert_eq!(data.len(), self.next_item_offset - next_valid_offset);
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(&data)?;
        
        self.file.set_len(data.len()as u64)?;
        self.file.sync_all()?;
    
        let mut normalized_offsets = vec![];
        for pos in self.offsets.iter().skip(index + 1) {
            normalized_offsets.push(pos - next_valid_offset)
        }
        
        self.offsets = normalized_offsets;
        self.next_item_offset = data.len();

        Ok(())
    }

    // get rid of log entries from index till the end of the log.
    // helps in keeping a log consistent with other logs.
    pub fn rebase(&mut self, index: usize) -> RaftResult<()> {
        let next_valid_offset = self.offsets[index];
        self.file.seek(SeekFrom::Start(next_valid_offset as u64))?;
        self.file.set_len(next_valid_offset as u64)?;
        self.file.sync_all()?;

        self.offsets.drain(index..);
        self.next_item_offset = next_valid_offset;
        
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.offsets.len()
    }

    fn load_offsets(file: &mut File,) -> RaftResult<(Vec<usize>, usize)> {
        let mut next_item_offset = 0usize ;
        let mut offsets = vec![];

        let mut data = vec![];
        file.read_to_end(&mut data)?;

        loop {
            file.seek(SeekFrom::Start(next_item_offset as u64))?;
            let length = match file.read_u64::<LittleEndian>() {
                Ok(v) => v as usize,
                Err(e) if e.kind() == ErrorKind::UnexpectedEof => break,
                Err(err) => return Err(RaftError::from(err))
            };
            offsets.push(next_item_offset); // means current position is a valid record
            next_item_offset += 8 + length;
        }

        Ok((offsets, next_item_offset))
    }
}


#[cfg(test)]
mod tests {
    use tempfile::tempdir;
    use crate::raft::error::RaftResult;

    use super::RaftLog;

    #[test]
    fn raf_log() -> RaftResult<()> {
        let temp_dir = tempdir()?;
        let log_path = temp_dir.path().join("raft_log.log");

        { // new log
            let mut raft_log = RaftLog::open(&log_path)?;

            raft_log.append::<Vec<u8>>(vec![1,1])?;
            raft_log.append::<Vec<u8>>(vec![2,2])?;
            raft_log.append::<Vec<u8>>(vec![3,3])?;

            assert_eq!(raft_log.len(), 3);
            assert_eq!(raft_log.get::<Vec<u8>>(0)?, vec![1,1]);

            raft_log.append::<Vec<u8>>(vec![4,4,4])?;
            raft_log.append::<Vec<u8>>(vec![5,5])?;
            raft_log.append::<Vec<u8>>(vec![6,6])?;
            raft_log.append::<Vec<u8>>(vec![7,7])?;

            assert_eq!(raft_log.len(), 7);
            assert_eq!(raft_log.get::<Vec<u8>>(3)?, vec![4,4,4]);
            assert_eq!(raft_log.get::<Vec<u8>>(4)?, vec![5,5]);
            assert_eq!(raft_log.get::<Vec<u8>>(6)?, vec![7,7]);

            raft_log.truncate(1)?;
            assert_eq!(raft_log.len(), 5);
            assert_eq!(raft_log.get::<Vec<u8>>(0)?, vec![3,3]);
            assert_eq!(raft_log.get::<Vec<u8>>(1)?, vec![4,4,4]);
        }

        { // existing log
            let mut raft_log = RaftLog::open(&log_path)?;
            assert_eq!(raft_log.len(), 5);
            assert_eq!(raft_log.get::<Vec<u8>>(0)?, vec![3,3]);
            assert_eq!(raft_log.get::<Vec<u8>>(4)?, vec![7,7]);

            raft_log.rebase(3)?;
            assert_eq!(raft_log.len(), 3);

            assert_eq!(raft_log.get::<Vec<u8>>(0)?, vec![3,3]);
            assert_eq!(raft_log.get::<Vec<u8>>(1)?, vec![4,4, 4]);
            assert_eq!(raft_log.get::<Vec<u8>>(2)?, vec![5,5]);
        }

        { // open rebased log
            let mut raft_log = RaftLog::open(log_path)?;
            assert_eq!(raft_log.len(), 3);

            assert_eq!(raft_log.get::<Vec<u8>>(0)?, vec![3,3]);
            assert_eq!(raft_log.get::<Vec<u8>>(1)?, vec![4,4, 4]);
            assert_eq!(raft_log.get::<Vec<u8>>(2)?, vec![5,5]);
        }
        
        Ok(())
    }
    
}
