use vintor::{decode,DecodeError};

use crate::packets::packet::{PacketError,PacketReadable};

#[derive(Debug)]
pub struct HandshakePacket{
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: i32

}
impl PacketReadable for HandshakePacket
{
    fn read(reader: &mut super::packet_reader::PacketReader) -> Result<Self, PacketError> {
    let protocol_version = reader.read_i32()?;
    let server_address = reader.read_string()?;
    let port_bytes = reader.read_bytes(2)?;  
    let server_port = u16::from_be_bytes([port_bytes[0], port_bytes[1]]);
    let next_state = reader.read_i32()?;

    Ok(HandshakePacket {
        protocol_version,
        server_address,
        server_port,
        next_state,
    })
}
}
impl From<DecodeError> for HandshakeError
{
    fn from(err :DecodeError) -> Self
    {
        HandshakeError::Decode(err)
    }
}
impl From<HandshakeError> for PacketError
{
    fn from(err: HandshakeError) -> Self
    {
        PacketError::Handshake(err)
    }
}
#[derive(Debug)]
pub enum HandshakeError
{
    Decode(DecodeError),
}
impl HandshakePacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<HandshakePacket,PacketError>
    {
        let mut cursor = 0;
        let (protocol_version, protocol_bytes_read) = decode(bytes)?;

        cursor +=protocol_bytes_read;

        let (address_len, address_len_bytes_read) = decode(&bytes[cursor..])?;
        cursor += address_len_bytes_read;

        let address_bytes = &bytes[cursor..cursor + address_len as usize];
        let address = std::str::from_utf8(address_bytes)
            .map_err(|_| PacketError::InvalidUtf8)?
            .to_string();

        cursor += address_len as usize;
        
        let port_bytes = &bytes[cursor..cursor + 2];
        let server_port = u16::from_be_bytes([port_bytes[0], port_bytes[1]]);
        cursor += 2;
        
        let (next_state, read) = decode(&bytes[cursor..])?;
        cursor += read;

        Ok(HandshakePacket
        {
            protocol_version: protocol_version,
            server_address: address,
            server_port: server_port,
            next_state: next_state
        })
    }
}
