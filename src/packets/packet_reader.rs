use vintor::{decode,decode_long,encode,encode_long,DecodeError,EncodeError};

#[derive(Debug)]
pub enum PacketReaderError {
    DecodeError(DecodeError),
    Other(String),
    UnexpectedEOF,
    InvalidUtf8
}

pub struct PacketReader<'a>{
    bytes: &'a [u8],
    pos: usize

}
impl From<DecodeError> for PacketReaderError
{
    fn from(value: DecodeError) -> Self {
        PacketReaderError::DecodeError(value)
}
}
impl<'a> PacketReader<'a>
{
    pub fn new(bytes: &'a [u8]) -> Self{
        PacketReader{
            bytes:bytes,
            pos: 0
        }
    }
    pub fn read_string(&mut self) -> Result<String,PacketReaderError>
    {
        let string_len = self.read_i32()?;


        let string_bytes = self.read_bytes(string_len as usize)?;

        String::from_utf8(string_bytes.to_vec())
        .map_err(|_| PacketReaderError::InvalidUtf8)
    }
    pub fn read_bytes(&mut self, size: usize) -> Result<&[u8],PacketReaderError>
    {
         if self.pos + size > self.bytes.len() {
        return Err(PacketReaderError::UnexpectedEOF);
    }
        let read_bytes = &self.bytes[self.pos..self.pos+size];
        self.pos += size;

        Ok(read_bytes)
    }
    pub fn read_i32(&mut self) -> Result<i32, PacketReaderError>
    {
        let (number,number_length) = decode(&self.bytes[self.pos..])?;
        self.pos += number_length as usize;
        
        Ok(number)
    }
}
