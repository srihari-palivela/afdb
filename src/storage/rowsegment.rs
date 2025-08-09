
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use anyhow::Result;
use std::fs::OpenOptions;
use std::io::{Write, Read};
use crate::types::VersionedRow;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RowSegmentMeta {
    pub rows: u64,
}

pub struct RowSegment {
    path: PathBuf,
}

impl RowSegment {
    pub fn create(path: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(path.parent().unwrap())?;
        let mut f = OpenOptions::new().create(true).write(true).open(&path)?;
        // placeholder meta
        let meta = RowSegmentMeta { rows: 0 };
        let bytes = bincode::serialize(&meta)?;
        f.write_all(&(bytes.len() as u32).to_le_bytes())?;
        f.write_all(&bytes)?;
        Ok(Self { path })
    }

    pub fn open(path: PathBuf) -> Result<Self> {
        Ok(Self { path })
    }

    pub fn append(&self, row: &VersionedRow) -> Result<()> {
        let mut f = OpenOptions::new().append(true).open(&self.path)?;
        let bytes = bincode::serialize(row)?;
        let len = bytes.len() as u32;
        f.write_all(&len.to_le_bytes())?;
        f.write_all(&bytes)?;
        Ok(())
    }

    pub fn iter(&self) -> Result<Vec<VersionedRow>> {
        let mut f = OpenOptions::new().read(true).open(&self.path)?;
        // read meta
        let mut len_buf = [0u8;4];
        f.read_exact(&mut len_buf)?;
        let meta_len = u32::from_le_bytes(len_buf) as usize;
        let mut meta_buf = vec![0u8; meta_len];
        f.read_exact(&mut meta_buf)?;
        // rows
        let mut out = Vec::new();
        loop {
            let mut len_buf = [0u8;4];
            if f.read_exact(&mut len_buf).is_err() { break; }
            let len = u32::from_le_bytes(len_buf) as usize;
            let mut buf = vec![0u8; len];
            f.read_exact(&mut buf)?;
            let rec: VersionedRow = bincode::deserialize(&buf)?;
            out.push(rec);
        }
        Ok(out)
    }
}
