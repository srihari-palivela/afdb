
use std::fs::{OpenOptions};
use std::io::{Write, Read, Seek, SeekFrom};
use std::path::PathBuf;
use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};

pub struct Wal {
    path: PathBuf,
}

impl Wal {
    pub fn open(path: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(path.parent().unwrap())?;
        Ok(Self { path })
    }

    pub fn append<T: Serialize>(&self, rec: &T) -> Result<u64> {
        let mut f = OpenOptions::new().create(true).append(true).open(&self.path)?;
        let bytes = bincode::serialize(rec)?;
        let len = bytes.len() as u32;
        f.write_all(&len.to_le_bytes())?;
        f.write_all(&bytes)?;
        f.flush()?;
        let pos = f.seek(SeekFrom::End(0))?;
        Ok(pos)
    }

    pub fn replay<T: DeserializeOwned>(&self) -> Result<Vec<T>> {
        let mut f = OpenOptions::new().read(true).create(true).open(&self.path)?;
        let mut out = Vec::new();
        loop {
            let mut len_buf = [0u8;4];
            if f.read_exact(&mut len_buf).is_err() { break; }
            let len = u32::from_le_bytes(len_buf) as usize;
            let mut buf = vec![0u8; len];
            f.read_exact(&mut buf)?;
            let rec: T = bincode::deserialize(&buf)?;
            out.push(rec);
        }
        Ok(out)
    }
}
