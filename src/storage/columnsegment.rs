
use serde::{Serialize, Deserialize};
use anyhow::Result;
use std::path::PathBuf;
use std::fs::OpenOptions;
use std::io::{Write, Read};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ZoneMap {
    pub min_len: u32,
    pub max_len: u32,
    #[serde(default)]
    pub min_ts: Option<u64>,
    #[serde(default)]
    pub max_ts: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Column {
    pub name: String,
    pub offsets: Vec<u64>,
    pub blob: Vec<u8>,
    pub zonemap: ZoneMap,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Segment {
    pub rows: u64,
    pub columns: Vec<Column>,
}

impl Segment {
    pub fn new() -> Self {
        Self { rows: 0, columns: Vec::new() }
    }

    pub fn add_string_column(&mut self, name: &str, values: &[String]) {
        let mut offsets = Vec::with_capacity(values.len());
        let mut blob = Vec::<u8>::new();
        let mut min_len = u32::MAX;
        let mut max_len = 0u32;

        let mut off: u64 = 0;
        for s in values {
            let bytes = s.as_bytes();
            let len = bytes.len() as u32;
            min_len = min_len.min(len);
            max_len = max_len.max(len);
            offsets.push(off);
            blob.extend_from_slice(&(len.to_le_bytes()));
            blob.extend_from_slice(bytes);
            off += 4 + bytes.len() as u64;
        }
        self.rows = values.len() as u64;
        self.columns.push(Column {
            name: name.to_string(),
            offsets,
            blob,
            zonemap: ZoneMap { min_len, max_len, min_ts: None, max_ts: None },
        });
    }

    pub fn write_to(&self, path: PathBuf) -> Result<()> {
        std::fs::create_dir_all(path.parent().unwrap())?;
        let mut f = OpenOptions::new().create(true).write(true).open(&path)?;
        let bytes = bincode::serialize(self)?;
        f.write_all(&(bytes.len() as u32).to_le_bytes())?;
        f.write_all(&bytes)?;
        Ok(())
    }

    pub fn read_from(path: PathBuf) -> Result<Self> {
        let mut f = OpenOptions::new().read(true).open(path)?;
        let mut len_buf = [0u8;4];
        f.read_exact(&mut len_buf)?;
        let meta_len = u32::from_le_bytes(len_buf) as usize;
        let mut buf = vec![0u8; meta_len];
        f.read_exact(&mut buf)?;
        let seg: Segment = bincode::deserialize(&buf)?;
        Ok(seg)
    }
}
