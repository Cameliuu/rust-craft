use crate::packets::packet::PacketError;
use vintor::{decode_long,DecodeError};

#[derive(Debug)]
pub struct PongPacket{
    number: i64
}
impl PongPacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<PongPacket, PacketError> {
        let (payload, bytes_read) = decode_long(bytes)
            .map_err(|_| PacketError::UnexpectedEOF)?;  // or map to your specific error


        Ok(PongPacket {number:payload})
    }
}

