
use vintor::{DecodeError,decode,EncodeError};

use crate::packets::{handhsake::{HandshakeError, HandshakePacket}, ping::PingPacket};
impl From<DecodeError> for PacketError {
    fn from(err: DecodeError) -> Self {
        PacketError::Decode(err)
    }
}
impl From<EncodeError> for PacketError {
    fn from(err: EncodeError) -> Self {
        PacketError::Encode(err)
    }
}
#[derive(Debug)]
pub enum PacketError
{
    Decode(DecodeError),
    IdNotSupported(u8),
    InvalidUtf8,
    Handshake(HandshakeError),
    UnexpectedEOF,
    Encode(EncodeError)
}
#[derive(Debug)]
pub enum Packet
{
    Handshake(HandshakePacket),
    Ping(PingPacket)
}

impl Packet {
    pub fn read_from_bytes(bytes: &[u8]) -> Result<Packet, PacketError> {
        if bytes.is_empty() {
            return Err(PacketError::UnexpectedEOF);
        }
        let mut cursor = 0;

        let (packet_length, bytes_read) = decode(&bytes)?;
        cursor += bytes_read;

        if bytes.len() < cursor + packet_length as usize {
            return Err(PacketError::UnexpectedEOF);
        }

        let packet_data = &bytes[cursor..cursor + packet_length as usize];

        let (packet_id, id_bytes_read) = decode(packet_data)?;

        if id_bytes_read > packet_data.len() {
            return Err(PacketError::UnexpectedEOF);
        }

        let payload = &packet_data[id_bytes_read..];

        match packet_id {
            0 => {
                let handshake_packet = HandshakePacket::from_bytes(payload)?;
                Ok(Packet::Handshake(handshake_packet))
            },
            1 => {
                let pong_packet = PingPacket::from_bytes(packet_length,packet_id as u8,payload)?;
                Ok(Packet::Ping(pong_packet))
            },
            _ => Err(PacketError::IdNotSupported(packet_id as u8)),
        }
    }
}



