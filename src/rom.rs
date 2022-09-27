use crate::Result;
use std::{fs, path::Path};

pub struct Rom {
    pub extension: String,
    pub data: Vec<u8>,
}

impl Rom {
    pub fn load_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let extension = path
            .as_ref()
            .extension()
            .ok_or("Invalid file extension")?
            .to_str()
            .ok_or("Invalid file name")?
            .to_owned();

        let file = fs::read(path)?;

        Ok(Self {
            data: file,
            extension,
        })
    }

    pub fn write_file<P: AsRef<Path>>(self, filename: P) -> Result<()> {
        fs::write(
            format!(
                "./{}.{}",
                filename.as_ref().to_str().ok_or("invalid file name")?,
                self.extension
            ),
            self.data,
        )?;
        Ok(())
    }
}
