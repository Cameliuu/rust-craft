use crate::packets::packet::PacketError;
use vintor::{decode_long,DecodeError};

#[derive(Debug)]
pub struct PingPacket{
    pub total_length: i32,
    pub packet_id:u8,
    pub bytes: Vec<u8>
}
impl PingPacket {
    pub fn from_bytes(total_length:i32,packet_id:u8,bytes: &[u8]) -> Result<PingPacket, PacketError> {
        let mut ping_bytes = Vec::new();
        ping_bytes.extend(bytes);
        Ok(PingPacket{
            total_length:total_length,
            packet_id:packet_id,
            bytes: ping_bytes})
    }
}

