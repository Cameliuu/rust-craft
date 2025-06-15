use crate::packets::packet::PacketError;
use vintor::{decode,DecodeError};
use uuid::Uuid;


#[derive(Debug  )]
pub struct LoginAckPacket{}
impl LoginAckPacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<LoginAckPacket,PacketError>
    {
        Ok(LoginAckPacket{})
    }
}
#[derive(Debug)]
pub struct LoginStartPacket
{
    pub player_name: String,
    pub uuid: Uuid 
}

impl LoginStartPacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<LoginStartPacket,PacketError>
    {
        let mut cursor = 0;
        let (player_name_length,player_name_read_bytes) = decode(bytes)?;
        
        cursor += player_name_read_bytes;
        let player_name_bytes = &bytes[cursor..cursor+player_name_length as usize];
        let player_name = std::str::from_utf8(player_name_bytes).expect("[ ! ] FAILED TO PARSE PLAYER NAME")
            .to_string();
        
        cursor += player_name_length as usize;

        let slice = &bytes[cursor..cursor + 16];
let uuid = Uuid::from_bytes(slice.try_into().map_err(|_| PacketError::UnexpectedEOF)?);

        Ok(LoginStartPacket
            {
                player_name:player_name,
                uuid: uuid
            })

    }
}
