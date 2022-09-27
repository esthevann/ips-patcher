use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Record {
    size: u16,
    offset: [u8; 3],
    pub data: Vec<u8>
}

impl Record {
    pub fn new(size: u16, offset: [u8; 3], data: Vec<u8> ) -> Self {
        Self { size, offset, data, }
    }

    pub fn get_offset(&self) -> (u32, u32) {
        let mut c = Cursor::new(self.offset.to_vec());
        let offset = c.read_u24::<BigEndian>().unwrap();
        (offset, offset + u32::from(self.size))
    }

}