use std::{
    fmt::Display,
    fs,
    io::{BufReader, Read},
    path::Path,
};

use crate::{record::Record, rom::Rom, Error, Result};

#[derive(Debug)]
pub struct Ips {
    header: String,
    records: Vec<Record>,
    eof: String,
}

impl Ips {
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = fs::read(path)?;
        file.as_slice().try_into()
    }

    fn new(header: String, records: Vec<Record>, eof: String) -> Self {
        Self {
            header,
            records,
            eof,
        }
    }

    pub fn apply_patch(self, game: Rom) -> Rom {
        let mut data = game.data;
        for i in self.records.into_iter() {
            let offset = i.get_offset();
            data[(offset.0 as usize)..(offset.1 as usize)]
                .copy_from_slice(&i.data[..(offset.1 as usize - offset.0 as usize)]);
        }
        Rom { data, extension: game.extension }
    }
}

impl TryFrom<&[u8]> for Ips {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let header = value.get(0..5).ok_or("Missig header")?;
        let header = String::from_utf8_lossy(header);
        if header != "PATCH" {
            return Err("Invalid header".into());
        }

        let records = value.get(5..).ok_or("Missing records")?;
        let mut reader = BufReader::new(records);
        let mut chunks: Vec<Record> = Vec::new();
        let eof = loop {
            let mut offset = [0; 3];

            if let Err(e) = reader.read_exact(&mut offset) {
                return Err(Box::new(e));
            }

            let eof = String::from_utf8_lossy(&offset).to_string();
            if eof == "EOF" {
                break eof;
            }

            let mut size = [0; 2];
            if let Err(e) = reader.read_exact(&mut size) {
                return Err(Box::new(e));
            }

            let size = u16::from_be_bytes(size);

            if size == 0 {
                let mut rle_size = [0; 2];
                if let Err(e) = reader.read_exact(&mut rle_size) {
                    return Err(Box::new(e));
                }
                let rle_size = u16::from_be_bytes(rle_size);
                let mut value = [0];
                if let Err(e) = reader.read_exact(&mut value) {
                    return Err(Box::new(e));
                }
                let mut data = Vec::new();
                for _i in 0..rle_size {
                    data.push(value[0])
                }

                chunks.push(Record::new(rle_size, offset, data));
            } else {
                let mut data = vec![0; size as usize];
                if let Err(e) = reader.read_exact(&mut data) {
                    return Err(Box::new(e));
                }

                if data.len() != size as usize {
                    return Err("Data and length doesn't match in record".into());
                }

                chunks.push(Record::new(size, offset, data));
            }
        };

        Ok(Ips::new(header.to_string(), chunks, eof))
    }
}

impl Display for Ips {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "header: {}, records: {}, eof: {}",
            self.header,
            self.records.len(),
            self.eof
        )
    }
}
