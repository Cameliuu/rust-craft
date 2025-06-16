
use crate::packets::packet::PacketError;

#[derive(Debug)]
pub struct StatusRequest;

impl StatusRequest {
    pub fn from_bytes(_payload: &[u8]) -> Result<StatusRequest, PacketError> {
        Ok(StatusRequest)
    }
}

